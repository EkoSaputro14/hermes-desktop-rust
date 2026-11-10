//! Core domain models for Hermes Desktop (Native Rust Edition)
//! Mirrors and upgrades the TypeScript/SQLite schema from fathah/hermes-desktop.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LlmProvider {
    Gemini,
    OpenRouter,
    Anthropic,
    OpenAi,
    Ollama,
    LmStudio,
    NousHermes,
    Groq,
    DeepSeek,
    Mistral,
    TogetherAi,
}

impl LlmProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gemini => "Gemini",
            Self::OpenRouter => "OpenRouter",
            Self::Anthropic => "Anthropic",
            Self::OpenAi => "OpenAI",
            Self::Ollama => "Ollama",
            Self::LmStudio => "LM Studio",
            Self::NousHermes => "Nous Hermes API",
            Self::Groq => "Groq",
            Self::DeepSeek => "DeepSeek",
            Self::Mistral => "Mistral",
            Self::TogetherAi => "Together AI",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub id: String,
    pub name: String,
    pub hermes_dir: String,
    pub provider: LlmProvider,
    pub model_name: String,
    pub endpoint_url: String,
    pub system_prompt: String,
    pub context_window: usize,
    pub honcho_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub profile_id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub token_count: usize,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionReply {
    pub content: String,
    pub token_count: usize,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryCategory {
    Fact,
    Preference,
    ProjectContext,
    HonchoSync,
    Directive,
}

impl MemoryCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fact => "Fact",
            Self::Preference => "Preference",
            Self::ProjectContext => "Project Context",
            Self::HonchoSync => "Honcho Sync",
            Self::Directive => "Directive",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub profile_id: String,
    pub category: MemoryCategory,
    pub content: String,
    pub importance_score: u8,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsetItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub plugin_path: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Scheduled => "Scheduled",
            Self::Running => "Running",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub title: String,
    pub cron_expression: String,
    pub profile_id: String,
    pub prompt_payload: String,
    pub status: TaskStatus,
    pub last_run_at: String,
    pub next_run_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub id: String,
    pub platform: String,
    pub bot_handle: String,
    pub linked_profile_id: String,
    pub enabled: bool,
    pub connection_status: String,
}
