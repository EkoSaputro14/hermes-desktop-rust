//! Hermes One Desktop — Native Rust GUI Edition (Iced 0.13 + Tokio + rusqlite)
//! Replicates the exact 1:1 layout and dark theme of fathah/hermes-desktop.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod icons;
mod message;
mod models;
mod services;
mod theme;
mod views;

use std::sync::Arc;
use iced::widget::{column, container, row};
use iced::{Element, Length, Subscription, Task, Theme, window};
use tracing::{info, error};

use crate::db::HermesDatabase;
use crate::message::{ActiveTab, Message, SseChunk};
use crate::models::{
    AgentProfile, ChatMessage, ChatSession, GatewayConfig, MemoryCategory,
    MemoryEntry, ScheduledTask, ToolsetItem,
};
use crate::services::hermes_cli::HermesCliManager;
use crate::services::sse_client::stream_agent_completion;

pub struct HermesApp {
    pub db: Arc<HermesDatabase>,
    pub active_tab: ActiveTab,
    pub profiles: Vec<AgentProfile>,
    pub active_profile_id: String,
    pub sessions: Vec<ChatSession>,
    pub active_session_id: String,
    pub messages: Vec<ChatMessage>,
    pub prompt_input: String,
    pub is_streaming: bool,
    pub streaming_buffer: String,
    pub memories: Vec<MemoryEntry>,
    pub memory_search_query: String,
    pub memory_category_filter: Option<MemoryCategory>,
    pub new_memory_input: String,
    pub toolsets: Vec<ToolsetItem>,
    pub scheduled_tasks: Vec<ScheduledTask>,
    pub new_task_title: String,
    pub new_task_cron: String,
    pub new_task_prompt: String,
    pub gateways: Vec<GatewayConfig>,
    pub status_banner: String,
}

impl HermesApp {
    pub fn new() -> (Self, Task<Message>) {
        let db = Arc::new(
            HermesDatabase::init().expect("Failed to initialize ~/.hermes/hermes_desktop.db"),
        );

        let profiles = db.list_profiles().unwrap_or_default();
        let active_profile_id = profiles
            .first()
            .map(|p| p.id.clone())
            .unwrap_or_else(|| "prof-default".to_string());

        let mut sessions = db.list_sessions(&active_profile_id).unwrap_or_default();
        if let Ok(fresh) = db.create_session(&active_profile_id, "New Chat") {
            sessions.insert(0, fresh);
        }
        let active_session_id = sessions
            .first()
            .map(|s| s.id.clone())
            .unwrap_or_else(|| "sess-welcome".to_string());

        let messages = db.list_messages(&active_session_id).unwrap_or_default();
        let memories = db.list_memories(&active_profile_id).unwrap_or_default();
        let toolsets = db.list_toolsets().unwrap_or_default();
        let scheduled_tasks = db.list_scheduled_tasks().unwrap_or_default();
        let gateways = db.list_gateways().unwrap_or_default();

        let app = Self {
            db,
            active_tab: ActiveTab::Chat,
            profiles,
            active_profile_id,
            sessions,
            active_session_id,
            messages,
            prompt_input: String::new(),
            is_streaming: false,
            streaming_buffer: String::new(),
            memories,
            memory_search_query: String::new(),
            memory_category_filter: None,
            new_memory_input: String::new(),
            toolsets,
            scheduled_tasks,
            new_task_title: String::new(),
            new_task_cron: "0 */6 * * *".to_string(),
            new_task_prompt: String::new(),
            gateways,
            status_banner: "local".to_string(),
        };

        (
            app,
            Task::perform(HermesCliManager::verify_environment(), Message::HermesEnvChecked),
        )
    }

    pub fn title(&self) -> String {
        "Hermes One Desktop".to_string()
    }

    pub fn theme(&self) -> Theme {
        theme::hermes_dark_theme()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SwitchTab(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::PromptInputChanged(value) => {
                self.prompt_input = value;
                Task::none()
            }
            Message::PrefillSuggestion(prompt) => {
                self.prompt_input = prompt;
                Task::none()
            }
            Message::SubmitPrompt => {
                let prompt = self.prompt_input.trim().to_string();
                if prompt.is_empty() || self.is_streaming {
                    return Task::none();
                }
                self.prompt_input.clear();
                self.is_streaming = true;
                self.streaming_buffer.clear();

                if let Ok(user_msg) = self.db.insert_message(&self.active_session_id, "user", &prompt, 0) {
                    self.messages.push(user_msg);
                }

                let profile = self
                    .profiles
                    .iter()
                    .find(|p| p.id == self.active_profile_id)
                    .cloned();

                if let Some(active_profile) = profile {
                    let history = self.messages.clone();
                    let memories = self.memories.clone();
                    return Task::perform(
                        async move {
                            stream_agent_completion(active_profile, history, memories, prompt).await
                        },
                        Message::StreamCompleted,
                    );
                }
                Task::none()
            }
            Message::SseTick(chunk) => {
                match chunk {
                    SseChunk::Delta(token) => {
                        self.streaming_buffer.push_str(&token);
                    }
                    SseChunk::ToolInvocation(tool_name) => {
                        self.status_banner = format!("tool: {}", tool_name);
                    }
                }
                Task::none()
            }
            Message::StreamCompleted(result) => {
                self.is_streaming = false;
                match result {
                    Ok(reply) => {
                        if let Ok(saved) = self.db.insert_message(
                            &self.active_session_id,
                            "assistant",
                            &reply.content,
                            reply.token_count,
                        ) {
                            self.messages.push(saved);
                        }
                        self.streaming_buffer.clear();
                    }
                    Err(err) => {
                        error!("Agent stream error: {}", err);
                    }
                }
                Task::none()
            }
            Message::SelectSession(session_id) => {
                self.active_session_id = session_id.clone();
                self.active_tab = ActiveTab::Chat;
                self.messages = self.db.list_messages(&session_id).unwrap_or_default();
                Task::none()
            }
            Message::CreateSession => {
                if let Ok(session) = self
                    .db
                    .create_session(&self.active_profile_id, "New Chat")
                {
                    self.active_session_id = session.id.clone();
                    self.sessions.insert(0, session);
                    self.messages.clear();
                    self.active_tab = ActiveTab::Chat;
                }
                Task::none()
            }
            Message::DeleteSession(session_id) => {
                if self.sessions.len() <= 1 {
                    return Task::none();
                }
                let _ = self.db.delete_session(&session_id);
                self.sessions.retain(|s| s.id != session_id);
                if self.active_session_id == session_id {
                    self.active_session_id = self
                        .sessions
                        .first()
                        .map(|s| s.id.clone())
                        .unwrap_or_default();
                    self.messages = self
                        .db
                        .list_messages(&self.active_session_id)
                        .unwrap_or_default();
                }
                Task::none()
            }
            Message::SelectProfile(profile_id) => {
                self.active_profile_id = profile_id.clone();
                self.sessions = self.db.list_sessions(&profile_id).unwrap_or_default();
                self.memories = self.db.list_memories(&profile_id).unwrap_or_default();
                if let Some(first) = self.sessions.first() {
                    self.active_session_id = first.id.clone();
                    self.messages = self.db.list_messages(&first.id).unwrap_or_default();
                }
                Task::none()
            }
            Message::VerifyHermesInstall => {
                Task::perform(HermesCliManager::verify_environment(), Message::HermesEnvChecked)
            }
            Message::HermesEnvChecked(summary) => {
                self.status_banner = summary;
                Task::none()
            }
            Message::MemorySearchChanged(query) => {
                self.memory_search_query = query;
                Task::none()
            }
            Message::NewMemoryInputChanged(val) => {
                self.new_memory_input = val;
                Task::none()
            }
            Message::AddMemoryEntry => {
                let content = self.new_memory_input.trim().to_string();
                if !content.is_empty() {
                    if let Ok(entry) = self.db.insert_memory(
                        &self.active_profile_id,
                        MemoryCategory::Fact,
                        &content,
                        85,
                    ) {
                        self.memories.insert(0, entry);
                        self.new_memory_input.clear();
                    }
                }
                Task::none()
            }
            Message::DeleteMemoryEntry(memory_id) => {
                let _ = self.db.delete_memory(&memory_id);
                self.memories.retain(|m| m.id != memory_id);
                Task::none()
            }
            Message::ToggleToolset(tool_id, enabled) => {
                let _ = self.db.set_toolset_enabled(&tool_id, enabled);
                if let Some(tool) = self.toolsets.iter_mut().find(|t| t.id == tool_id) {
                    tool.enabled = enabled;
                }
                Task::none()
            }
            Message::AdvanceTaskStatus(task_id, next_status) => {
                let _ = self.db.update_task_status(&task_id, next_status.clone());
                if let Some(task) = self.scheduled_tasks.iter_mut().find(|t| t.id == task_id) {
                    task.status = next_status;
                }
                Task::none()
            }
            Message::ToggleGateway(gateway_id, enabled) => {
                let _ = self.db.set_gateway_enabled(&gateway_id, enabled);
                if let Some(gw) = self.gateways.iter_mut().find(|g| g.id == gateway_id) {
                    gw.enabled = enabled;
                    gw.connection_status = if enabled {
                        "Connected".to_string()
                    } else {
                        "Standby".to_string()
                    };
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = views::view_sidebar(self);
        let sessions_bar = views::view_active_sessions_bar(self);
        let status_bar = views::view_status_bar(self);

        let content = match self.active_tab {
            ActiveTab::Chat => views::view_chat(self),
            ActiveTab::Discover => views::view_discover(self),
            ActiveTab::Office => views::view_office(self),
            ActiveTab::Kanban => views::view_kanban(self),
            ActiveTab::Schedules => views::view_tasks(self),
            ActiveTab::Providers => views::view_profiles(self),
            ActiveTab::Gateways => views::view_gateways(self),
            ActiveTab::Toolsets => views::view_toolsets(self),
            ActiveTab::Memory => views::view_memory(self),
            ActiveTab::Profiles => views::view_profiles(self),
        };

        let main_column = container(
            column![sessions_bar, content]
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::main_container);

        let top_row = row![sidebar, main_column]
            .width(Length::Fill)
            .height(Length::Fill);

        container(column![top_row, status_bar].width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::main_container)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

pub fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter("hermes_desktop=info")
        .init();

    info!("Starting Hermes One Desktop (Native Rust Iced Runtime)");

    iced::application(
        "Hermes One Desktop",
        HermesApp::update,
        HermesApp::view,
    )
    .subscription(HermesApp::subscription)
    .theme(HermesApp::theme)
    .window(window::Settings {
        size: iced::Size::new(1360.0, 860.0),
        min_size: Some(iced::Size::new(960.0, 640.0)),
        ..Default::default()
    })
    .run_with(HermesApp::new)
}
