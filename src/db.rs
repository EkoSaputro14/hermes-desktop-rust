//! Embedded SQLite persistence layer using `rusqlite` (bundled C SQLite).
//! Replaces `better-sqlite3` from Electron without requiring node-gyp or V8 FFI marshaling.

use std::path::PathBuf;
use std::sync::Mutex;
use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::models::{
    AgentProfile, ChatMessage, ChatSession, GatewayConfig, LlmProvider,
    MemoryCategory, MemoryEntry, ScheduledTask, TaskStatus, ToolsetItem,
};

pub struct HermesDatabase {
    conn: Mutex<Connection>,
}

impl HermesDatabase {
    pub fn init() -> Result<Self> {
        let hermes_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".hermes");
        std::fs::create_dir_all(&hermes_dir)
            .context("Failed to create ~/.hermes directory")?;

        let db_path = hermes_dir.join("hermes_desktop.db");
        let conn = Connection::open(db_path)?;

        // High-concurrency WAL mode for concurrent cron workers & UI queries
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;",
        )?;

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                hermes_dir TEXT NOT NULL,
                provider TEXT NOT NULL,
                model_name TEXT NOT NULL,
                endpoint_url TEXT NOT NULL,
                system_prompt TEXT NOT NULL,
                context_window INTEGER NOT NULL DEFAULT 128000,
                honcho_enabled INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                profile_id TEXT NOT NULL,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                token_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                FOREIGN KEY(session_id) REFERENCES sessions(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                profile_id TEXT NOT NULL,
                category TEXT NOT NULL,
                content TEXT NOT NULL,
                importance_score INTEGER NOT NULL DEFAULT 80,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS toolsets (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                description TEXT NOT NULL,
                plugin_path TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS scheduled_tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                cron_expression TEXT NOT NULL,
                profile_id TEXT NOT NULL,
                prompt_payload TEXT NOT NULL,
                status TEXT NOT NULL,
                last_run_at TEXT NOT NULL,
                next_run_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS gateways (
                id TEXT PRIMARY KEY,
                platform TEXT NOT NULL,
                bot_handle TEXT NOT NULL,
                linked_profile_id TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 0,
                connection_status TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    pub fn list_profiles(&self) -> Result<Vec<AgentProfile>> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, name, hermes_dir, provider, model_name, endpoint_url, system_prompt, context_window, honcho_enabled
             FROM profiles ORDER BY name ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(AgentProfile {
                id: row.get(0)?,
                name: row.get(1)?,
                hermes_dir: row.get(2)?,
                provider: LlmProvider::Gemini,
                model_name: row.get(4)?,
                endpoint_url: row.get(5)?,
                system_prompt: row.get(6)?,
                context_window: row.get::<_, usize>(7)?,
                honcho_enabled: row.get::<_, i64>(8)? != 0,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn list_sessions(&self, profile_id: &str) -> Result<Vec<ChatSession>> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, profile_id, title, created_at, updated_at
             FROM sessions WHERE profile_id = ?1 ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map(params![profile_id], |row| {
            Ok(ChatSession {
                id: row.get(0)?,
                profile_id: row.get(1)?,
                title: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn create_session(&self, profile_id: &str, title: &str) -> Result<ChatSession> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let id = format!("sess-{}", Uuid::new_v4().simple());
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO sessions (id, profile_id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, profile_id, title, now, now],
        )?;
        Ok(ChatSession {
            id,
            profile_id: profile_id.to_string(),
            title: title.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn delete_session(&self, session_id: &str) -> Result<()> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        conn.execute("DELETE FROM sessions WHERE id = ?1", params![session_id])?;
        Ok(())
    }

    pub fn list_messages(&self, session_id: &str) -> Result<Vec<ChatMessage>> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, session_id, role, content, token_count, created_at
             FROM messages WHERE session_id = ?1 ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(ChatMessage {
                id: row.get(0)?,
                session_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                token_count: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn insert_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
        token_count: usize,
    ) -> Result<ChatMessage> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let id = format!("msg-{}", Uuid::new_v4().simple());
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO messages (id, session_id, role, content, token_count, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, session_id, role, content, token_count, now],
        )?;
        conn.execute(
            "UPDATE sessions SET updated_at = ?1 WHERE id = ?2",
            params![now, session_id],
        )?;
        Ok(ChatMessage {
            id,
            session_id: session_id.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            token_count,
            created_at: now,
        })
    }

    pub fn list_memories(&self, profile_id: &str) -> Result<Vec<MemoryEntry>> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, profile_id, category, content, importance_score, created_at
             FROM memories WHERE profile_id = ?1 ORDER BY importance_score DESC",
        )?;
        let rows = stmt.query_map(params![profile_id], |row| {
            Ok(MemoryEntry {
                id: row.get(0)?,
                profile_id: row.get(1)?,
                category: MemoryCategory::Fact,
                content: row.get(3)?,
                importance_score: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn insert_memory(
        &self,
        profile_id: &str,
        category: MemoryCategory,
        content: &str,
        importance_score: u8,
    ) -> Result<MemoryEntry> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let id = format!("mem-{}", Uuid::new_v4().simple());
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO memories (id, profile_id, category, content, importance_score, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, profile_id, category.as_str(), content, importance_score, now],
        )?;
        Ok(MemoryEntry {
            id,
            profile_id: profile_id.to_string(),
            category,
            content: content.to_string(),
            importance_score,
            created_at: now,
        })
    }

    pub fn delete_memory(&self, memory_id: &str) -> Result<()> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        conn.execute("DELETE FROM memories WHERE id = ?1", params![memory_id])?;
        Ok(())
    }

    pub fn list_toolsets(&self) -> Result<Vec<ToolsetItem>> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, name, category, description, plugin_path, enabled FROM toolsets ORDER BY name ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ToolsetItem {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                plugin_path: row.get(4)?,
                enabled: row.get::<_, i64>(5)? != 0,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn set_toolset_enabled(&self, tool_id: &str, enabled: bool) -> Result<()> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        conn.execute(
            "UPDATE toolsets SET enabled = ?1 WHERE id = ?2",
            params![if enabled { 1 } else { 0 }, tool_id],
        )?;
        Ok(())
    }

    pub fn list_scheduled_tasks(&self) -> Result<Vec<ScheduledTask>> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, title, cron_expression, profile_id, prompt_payload, status, last_run_at, next_run_at
             FROM scheduled_tasks ORDER BY next_run_at ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ScheduledTask {
                id: row.get(0)?,
                title: row.get(1)?,
                cron_expression: row.get(2)?,
                profile_id: row.get(3)?,
                prompt_payload: row.get(4)?,
                status: TaskStatus::Scheduled,
                last_run_at: row.get(6)?,
                next_run_at: row.get(7)?,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn update_task_status(&self, task_id: &str, status: TaskStatus) -> Result<()> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        conn.execute(
            "UPDATE scheduled_tasks SET status = ?1 WHERE id = ?2",
            params![status.as_str(), task_id],
        )?;
        Ok(())
    }

    pub fn list_gateways(&self) -> Result<Vec<GatewayConfig>> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, platform, bot_handle, linked_profile_id, enabled, connection_status
             FROM gateways ORDER BY platform ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(GatewayConfig {
                id: row.get(0)?,
                platform: row.get(1)?,
                bot_handle: row.get(2)?,
                linked_profile_id: row.get(3)?,
                enabled: row.get::<_, i64>(4)? != 0,
                connection_status: row.get(5)?,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn set_gateway_enabled(&self, gateway_id: &str, enabled: bool) -> Result<()> {
        let conn = self.conn.lock().expect("SQLite mutex poisoned");
        let status = if enabled { "Connected" } else { "Standby" };
        conn.execute(
            "UPDATE gateways SET enabled = ?1, connection_status = ?2 WHERE id = ?3",
            params![if enabled { 1 } else { 0 }, status, gateway_id],
        )?;
        Ok(())
    }
}
