//! Native GPU-rendered Iced 0.13 views for Hermes Desktop
//! Implements Sidebar, Top Bar, Chat SSE, Profiles, Memory, Toolsets, Scheduled Tasks, and Gateways.

use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Length};

use crate::HermesApp;
use crate::message::{ActiveTab, Message};
use crate::models::TaskStatus;

pub fn view_sidebar(app: &HermesApp) -> Element<'_, Message> {
    let nav_btn = |label: &'static str, tab: ActiveTab| {
        let is_active = app.active_tab == tab;
        let label_text = if is_active {
            format!("▸ {}", label)
        } else {
            format!("  {}", label)
        };
        button(text(label_text).size(14))
            .width(Length::Fill)
            .padding([10, 14])
            .on_press(Message::SwitchTab(tab))
    };

    let nav_list = column![
        text("Hermes Desktop").size(18),
        text("Native Rust · Iced 0.13").size(12),
        nav_btn("Agent Chat (SSE)", ActiveTab::Chat),
        nav_btn("Profiles & ~/.hermes", ActiveTab::Profiles),
        nav_btn("Memory (Honcho)", ActiveTab::Memory),
        nav_btn("Toolsets & Plugins", ActiveTab::Toolsets),
        nav_btn("Scheduled Tasks", ActiveTab::ScheduledTasks),
        nav_btn("Messaging Gateways", ActiveTab::Gateways),
    ]
    .spacing(8)
    .padding(16)
    .width(Length::Fixed(256.0));

    container(nav_list).height(Length::Fill).into()
}

pub fn view_top_bar(app: &HermesApp) -> Element<'_, Message> {
    let section_title = match app.active_tab {
        ActiveTab::Chat => "Workspace / Agent Chat & SSE Stream",
        ActiveTab::Profiles => "Workspace / Agent Profiles & ~/.hermes Environment",
        ActiveTab::Memory => "Workspace / Honcho Memory & Semantic Store",
        ActiveTab::Toolsets => "Workspace / Native Toolsets & ~/.hermes/plugins",
        ActiveTab::ScheduledTasks => "Workspace / Autonomous Cron Tasks & Kanban",
        ActiveTab::Gateways => "Workspace / External Messaging Gateways",
    };

    let bar = row![
        text(section_title).size(15),
        horizontal_space(),
        text(&app.status_banner).size(12),
        button(text("Verify ~/.hermes").size(12))
            .padding([6, 12])
            .on_press(Message::VerifyHermesInstall),
    ]
    .spacing(16)
    .align_y(Alignment::Center)
    .padding([12, 20]);

    container(bar).width(Length::Fill).into()
}

pub fn view_chat(app: &HermesApp) -> Element<'_, Message> {
    let mut session_col = column![
        button(text("+ New Session").size(13))
            .width(Length::Fill)
            .padding([8, 12])
            .on_press(Message::CreateSession)
    ]
    .spacing(6)
    .width(Length::Fixed(240.0))
    .padding(12);

    for sess in &app.sessions {
        session_col = session_col.push(
            row![
                button(text(&sess.title).size(13))
                    .width(Length::Fill)
                    .on_press(Message::SelectSession(sess.id.clone())),
                button(text("×").size(13))
                    .on_press(Message::DeleteSession(sess.id.clone())),
            ]
            .spacing(4),
        );
    }

    let mut msg_list = column![].spacing(12).padding(20);
    for msg in &app.messages {
        let prefix = if msg.role == "user" { "You" } else { "Hermes Agent" };
        msg_list = msg_list.push(
            column![
                text(format!("{} · {} tokens", prefix, msg.token_count)).size(12),
                text(&msg.content).size(14),
            ]
            .spacing(4),
        );
    }

    if app.is_streaming {
        msg_list = msg_list.push(
            column![
                text("Hermes Agent · Streaming SSE...").size(12),
                text(&app.streaming_buffer).size(14),
            ]
            .spacing(4),
        );
    }

    let composer = row![
        text_input(
            "Ask Hermes Agent, invoke a toolset, or query memory...",
            &app.prompt_input
        )
        .on_input(Message::PromptInputChanged)
        .on_submit(Message::SubmitPrompt)
        .padding(10)
        .size(14),
        button(text(if app.is_streaming { "Streaming..." } else { "Send" }).size(14))
            .padding([10, 18])
            .on_press(Message::SubmitPrompt),
    ]
    .spacing(10)
    .padding(16);

    let chat_main = column![scrollable(msg_list).height(Length::Fill), composer,]
        .width(Length::Fill)
        .height(Length::Fill);

    row![session_col, chat_main]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn view_profiles(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Agent Profiles & 11+ LLM Providers").size(18),
        text("Manage isolated ~/.hermes configurations, model endpoints, and system directives.")
            .size(13),
    ]
    .spacing(12)
    .padding(24);

    for profile in &app.profiles {
        let is_selected = profile.id == app.active_profile_id;
        list = list.push(
            row![
                column![
                    text(format!("{} ({})", profile.name, profile.provider.as_str())).size(15),
                    text(format!(
                        "{} · {} · Context: {} tokens",
                        profile.model_name, profile.hermes_dir, profile.context_window
                    ))
                    .size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
                button(text(if is_selected { "Active Profile" } else { "Activate" }).size(13))
                    .padding([8, 14])
                    .on_press(Message::SelectProfile(profile.id.clone())),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        );
    }

    scrollable(list).height(Length::Fill).into()
}

pub fn view_memory(app: &HermesApp) -> Element<'_, Message> {
    let header = column![
        text("Honcho Memory & Persistent Context Store").size(18),
        row![
            text_input("Filter memories by keyword...", &app.memory_search_query)
                .on_input(Message::MemorySearchChanged)
                .padding(8)
                .size(13),
            text_input("Add new persistent agent memory...", &app.new_memory_input)
                .on_input(Message::NewMemoryInputChanged)
                .padding(8)
                .size(13),
            button(text("Store Memory").size(13))
                .padding([8, 14])
                .on_press(Message::AddMemoryEntry),
        ]
        .spacing(10),
    ]
    .spacing(12);

    let mut items = column![header].spacing(10).padding(24);
    for mem in &app.memories {
        if !app.memory_search_query.is_empty()
            && !mem
                .content
                .to_lowercase()
                .contains(&app.memory_search_query.to_lowercase())
        {
            continue;
        }
        items = items.push(
            row![
                column![
                    text(format!(
                        "{} · Weight: {}/100 · {}",
                        mem.category.as_str(),
                        mem.importance_score,
                        mem.created_at
                    ))
                    .size(12),
                    text(&mem.content).size(14),
                ]
                .spacing(4)
                .width(Length::Fill),
                button(text("Delete").size(12))
                    .padding([6, 10])
                    .on_press(Message::DeleteMemoryEntry(mem.id.clone())),
            ]
            .spacing(12),
        );
    }

    scrollable(items).height(Length::Fill).into()
}

pub fn view_toolsets(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Native Toolsets & ~/.hermes/plugins").size(18),
        text("Enable or restrict autonomous capabilities and external plugins per agent session.")
            .size(13),
    ]
    .spacing(12)
    .padding(24);

    for tool in &app.toolsets {
        let tid = tool.id.clone();
        let next_state = !tool.enabled;
        list = list.push(
            row![
                column![
                    text(format!("{} · {}", tool.name, tool.category)).size(15),
                    text(format!("{} ({})", tool.description, tool.plugin_path)).size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
                button(text(if tool.enabled { "Enabled" } else { "Disabled" }).size(12))
                    .padding([6, 12])
                    .on_press(Message::ToggleToolset(tid, next_state)),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        );
    }

    scrollable(list).height(Length::Fill).into()
}

pub fn view_tasks(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Scheduled Cron Tasks & Autonomous Execution Board").size(18),
        text("Managed by background Tokio scheduler with SQLite WAL state persistence.").size(13),
    ]
    .spacing(12)
    .padding(24);

    for task in &app.scheduled_tasks {
        let tid = task.id.clone();
        list = list.push(
            row![
                column![
                    text(format!("{} · Cron: {}", task.title, task.cron_expression)).size(15),
                    text(format!(
                        "Status: {} · Next run: {}",
                        task.status.as_str(),
                        task.next_run_at
                    ))
                    .size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
                button(text("Run Now").size(12))
                    .padding([6, 12])
                    .on_press(Message::AdvanceTaskStatus(tid, TaskStatus::Completed)),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        );
    }

    scrollable(list).height(Length::Fill).into()
}

pub fn view_gateways(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Messaging Gateways (Telegram, Discord, Slack)").size(18),
        text("Bridge external messaging channels directly to local Hermes Agent profiles.").size(13),
    ]
    .spacing(12)
    .padding(24);

    for gw in &app.gateways {
        let gid = gw.id.clone();
        let next_state = !gw.enabled;
        list = list.push(
            row![
                column![
                    text(format!("{} · {}", gw.platform, gw.bot_handle)).size(15),
                    text(format!(
                        "Status: {} · Linked Profile: {}",
                        gw.connection_status, gw.linked_profile_id
                    ))
                    .size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
                button(text(if gw.enabled { "Bridge Active" } else { "Standby" }).size(12))
                    .padding([6, 12])
                    .on_press(Message::ToggleGateway(gid, next_state)),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        );
    }

    scrollable(list).height(Length::Fill).into()
}
