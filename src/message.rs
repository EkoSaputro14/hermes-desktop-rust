//! Strongly-typed Elm Architecture Messages for Hermes One Desktop (Iced 0.13)
//! Mirrors the exact navigation and actions of fathah/hermes-desktop.

use crate::models::{AgentCompletionReply, TaskStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Chat,
    Discover,
    Office,
    Kanban,
    Schedules,
    Providers,
    Gateways,
    Toolsets,
    Memory,
    Profiles,
}

#[derive(Debug, Clone)]
pub enum SseChunk {
    Delta(String),
    ToolInvocation(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchTab(ActiveTab),
    PromptInputChanged(String),
    PrefillSuggestion(String),
    SubmitPrompt,
    SseTick(SseChunk),
    StreamCompleted(Result<AgentCompletionReply, String>),
    SelectSession(String),
    CreateSession,
    DeleteSession(String),
    SelectProfile(String),
    VerifyHermesInstall,
    HermesEnvChecked(String),
    MemorySearchChanged(String),
    NewMemoryInputChanged(String),
    AddMemoryEntry,
    DeleteMemoryEntry(String),
    ToggleToolset(String, bool),
    AdvanceTaskStatus(String, TaskStatus),
    ToggleGateway(String, bool),
}
