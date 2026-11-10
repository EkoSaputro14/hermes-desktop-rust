//! Elm-architecture Message definitions for Hermes Desktop (Iced 0.13)
//! Replaces Electron's ipcMain.handle / ipcRenderer.invoke channels with zero-copy Rust enums.

use crate::models::{CompletionReply, TaskStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Chat,
    Profiles,
    Memory,
    Toolsets,
    ScheduledTasks,
    Gateways,
}

#[derive(Debug, Clone)]
pub enum SseChunk {
    Delta(String),
    ToolInvocation(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    SwitchTab(ActiveTab),

    // SSE Chat & Session Management
    PromptInputChanged(String),
    SubmitPrompt,
    SseTick(SseChunk),
    StreamCompleted(Result<CompletionReply, String>),
    SelectSession(String),
    CreateSession,
    DeleteSession(String),

    // Profile & ~/.hermes CLI Management
    SelectProfile(String),
    VerifyHermesInstall,
    HermesEnvChecked(String),

    // Memory System (Honcho & Local SQLite)
    MemorySearchChanged(String),
    NewMemoryInputChanged(String),
    AddMemoryEntry,
    DeleteMemoryEntry(String),

    // Toolsets & Plugins (~/.hermes/plugins/)
    ToggleToolset(String, bool),

    // Cron Scheduled Tasks & Kanban Board
    AdvanceTaskStatus(String, TaskStatus),

    // Messaging Gateways (Telegram, Discord, Slack)
    ToggleGateway(String, bool),
}
