//! Native GPU-rendered Iced 0.13 views replicating fathah/hermes-desktop 1:1
//! Layout: Left Sidebar (New Chat, Pinned Nav, Recent Chats, Footer Icons, Profile Switcher)
//! + Main Content (ActiveSessionsBar + Active Screen) + Bottom StatusBar (26px).

use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, vertical_space,
};
use iced::{Alignment, Element, Length};

use crate::HermesApp;
use crate::message::{ActiveTab, Message};
use crate::models::TaskStatus;

pub fn view_sidebar(app: &HermesApp) -> Element<'_, Message> {
    let new_chat_btn = button(text("+  New Chat").size(13))
        .width(Length::Fill)
        .padding([8, 12])
        .on_press(Message::CreateSession);

    let nav_btn = |label: &'static str, tab: ActiveTab| {
        let is_active = app.active_tab == tab;
        let label_text = if is_active {
            format!("●  {}", label)
        } else {
            format!("   {}", label)
        };
        button(text(label_text).size(13))
            .width(Length::Fill)
            .padding([7, 12])
            .on_press(Message::SwitchTab(tab))
    };

    let pinned_nav = column![
        new_chat_btn,
        nav_btn("Discover", ActiveTab::Discover),
        nav_btn("Office", ActiveTab::Office),
        nav_btn("Kanban", ActiveTab::Kanban),
        nav_btn("Schedules", ActiveTab::Schedules),
    ]
    .spacing(3);

    let mut recent_chats = column![text("CHATS").size(10)].spacing(4);
    for sess in &app.sessions {
        let is_current = app.active_tab == ActiveTab::Chat && app.active_session_id == sess.id;
        let prefix = if is_current { "▸ " } else { "  " };
        recent_chats = recent_chats.push(
            row![
                button(text(format!("{}{}", prefix, sess.title)).size(12))
                    .width(Length::Fill)
                    .padding([5, 8])
                    .on_press(Message::SelectSession(sess.id.clone())),
                button(text("×").size(12))
                    .padding([4, 7])
                    .on_press(Message::DeleteSession(sess.id.clone())),
            ]
            .spacing(3)
            .align_y(Alignment::Center),
        );
    }

    let footer_actions = row![
        button(text("Key").size(11))
            .padding([5, 7])
            .on_press(Message::SwitchTab(ActiveTab::Providers)),
        button(text("Gwy").size(11))
            .padding([5, 7])
            .on_press(Message::SwitchTab(ActiveTab::Gateways)),
        button(text("Cap").size(11))
            .padding([5, 7])
            .on_press(Message::SwitchTab(ActiveTab::Toolsets)),
        button(text("Mem").size(11))
            .padding([5, 7])
            .on_press(Message::SwitchTab(ActiveTab::Memory)),
        button(text("Prf").size(11))
            .padding([5, 7])
            .on_press(Message::SwitchTab(ActiveTab::Profiles)),
    ]
    .spacing(4);

    let active_profile_name = app
        .profiles
        .iter()
        .find(|p| p.id == app.active_profile_id)
        .map(|p| p.name.as_str())
        .unwrap_or("default");

    let profile_switcher = button(text(format!("Profile: {}", active_profile_name)).size(12))
        .width(Length::Fill)
        .padding([7, 10])
        .on_press(Message::SwitchTab(ActiveTab::Profiles));

    let sidebar_col = column![
        pinned_nav,
        scrollable(recent_chats).height(Length::Fill),
        footer_actions,
        profile_switcher,
    ]
    .spacing(12)
    .padding(10)
    .width(Length::Fixed(220.0));

    container(sidebar_col).height(Length::Fill).into()
}

pub fn view_active_sessions_bar(app: &HermesApp) -> Element<'_, Message> {
    let mut tabs = row![].spacing(4).align_y(Alignment::Center);
    for sess in app.sessions.iter().take(5) {
        tabs = tabs.push(
            row![
                button(text(&sess.title).size(12))
                    .padding([5, 10])
                    .on_press(Message::SelectSession(sess.id.clone())),
                button(text("×").size(11))
                    .padding([5, 6])
                    .on_press(Message::DeleteSession(sess.id.clone())),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        );
    }
    tabs = tabs.push(
        button(text("+").size(12))
            .padding([5, 9])
            .on_press(Message::CreateSession),
    );

    container(tabs).width(Length::Fill).padding([6, 12]).into()
}

pub fn view_status_bar(app: &HermesApp) -> Element<'_, Message> {
    let active_model = app
        .profiles
        .iter()
        .find(|p| p.id == app.active_profile_id)
        .map(|p| p.model_name.as_str())
        .unwrap_or("hermes-3-llama-3.1-405b");

    let enabled_skills = app.toolsets.iter().filter(|t| t.enabled).count();

    let bar = row![
        text(format!(
            "● gateway · local · {} · {} skills",
            active_model, enabled_skills
        ))
        .size(11),
        horizontal_space(),
        text("/ commands · Ctrl, settings").size(11),
    ]
    .align_y(Alignment::Center)
    .padding([4, 12]);

    container(bar).width(Length::Fill).into()
}

pub fn view_chat(app: &HermesApp) -> Element<'_, Message> {
    let body: Element<'_, Message> = if app.messages.is_empty() && !app.is_streaming {
        let suggestion_btn = |label: &'static str, prompt: &'static str| {
            button(text(label).size(12))
                .padding([7, 14])
                .on_press(Message::PrefillSuggestion(prompt.to_string()))
        };

        let suggestions_row1 = row![
            suggestion_btn("Search the web", "Search the web for the latest developments in "),
            suggestion_btn("Set a reminder", "Remind me in 30 minutes to "),
            suggestion_btn("Summarize emails", "Check my recent emails and summarize anything important"),
        ]
        .spacing(8);

        let suggestions_row2 = row![
            suggestion_btn("Write a script", "Write a Python script that "),
            suggestion_btn("Schedule a cron job", "Set up a daily scheduled task at 9am to "),
            suggestion_btn("Analyze data", "Help me analyze and visualize "),
        ]
        .spacing(8);

        let empty_col = column![
            vertical_space(),
            text("HERMES ONE").size(16),
            text("How can I help you today?").size(22),
            text("Ask me to write code, answer questions, search the web, and more").size(13),
            suggestions_row1,
            suggestions_row2,
            vertical_space(),
        ]
        .spacing(12)
        .align_x(Alignment::Center)
        .width(Length::Fill);

        container(empty_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    } else {
        let mut msg_list = column![].spacing(12).padding(20);
        for msg in &app.messages {
            let prefix = if msg.role == "user" { "You" } else { "Hermes" };
            msg_list = msg_list.push(
                column![
                    text(format!("{} · {}", prefix, msg.created_at)).size(11),
                    text(&msg.content).size(14),
                ]
                .spacing(4),
            );
        }

        if app.is_streaming {
            msg_list = msg_list.push(
                column![
                    text("Hermes · Thinking...").size(11),
                    text(&app.streaming_buffer).size(14),
                ]
                .spacing(4),
            );
        }

        scrollable(msg_list).height(Length::Fill).into()
    };

    let active_model = app
        .profiles
        .iter()
        .find(|p| p.id == app.active_profile_id)
        .map(|p| p.model_name.as_str())
        .unwrap_or("hermes-3-llama-3.1-405b");

    let composer = column![
        row![
            text_input("Ask anything", &app.prompt_input)
                .on_input(Message::PromptInputChanged)
                .on_submit(Message::SubmitPrompt)
                .padding(12)
                .size(14),
            button(text(if app.is_streaming { "■" } else { "↑" }).size(14))
                .padding([10, 16])
                .on_press(Message::SubmitPrompt),
        ]
        .spacing(8),
        row![
            text(format!("Model: {}  ·  Effort: medium", active_model)).size(11),
            horizontal_space(),
            text(&app.status_banner).size(11),
        ],
    ]
    .spacing(6)
    .padding([12, 24]);

    column![body, composer]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn view_discover(app: &HermesApp) -> Element<'_, Message> {
    view_toolsets(app)
}

pub fn view_office(app: &HermesApp) -> Element<'_, Message> {
    let mut col = column![
        text("Hermes Agent Office").size(20),
        text("Live multi-agent station status across Coordinator, Research, Code Forge, and Memory Vault.").size(13),
    ]
    .spacing(12)
    .padding(24);

    for prof in &app.profiles {
        col = col.push(
            row![
                column![
                    text(format!("Station · {}", prof.name)).size(15),
                    text(format!("Model: {} · Dir: {}", prof.model_name, prof.hermes_dir)).size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
                button(text("Switch to Agent").size(12))
                    .padding([6, 12])
                    .on_press(Message::SelectProfile(prof.id.clone())),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        );
    }

    scrollable(col).height(Length::Fill).into()
}

pub fn view_kanban(app: &HermesApp) -> Element<'_, Message> {
    view_tasks(app)
}

pub fn view_profiles(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Profiles & Providers").size(20),
        text("Manage isolated ~/.hermes/profiles directories, API endpoints, and default models.")
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
                button(text(if is_selected { "Active" } else { "Switch" }).size(12))
                    .padding([6, 14])
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
        text("Memory & Identity (MEMORY.md / SOUL.md)").size(20),
        row![
            text_input("Filter memories...", &app.memory_search_query)
                .on_input(Message::MemorySearchChanged)
                .padding(8)
                .size(13),
            text_input("Add memory entry...", &app.new_memory_input)
                .on_input(Message::NewMemoryInputChanged)
                .padding(8)
                .size(13),
            button(text("Add Memory").size(13))
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
                    text(format!("{} · {}", mem.category.as_str(), mem.created_at)).size(11),
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
        text("Capabilities · Toolsets, Skills & MCP Servers").size(20),
        text("Enable or disable built-in capabilities and ~/.hermes/plugins.")
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
        text("Schedules & Cron Automation").size(20),
        text("Automate recurring prompts and background agent tasks.").size(13),
    ]
    .spacing(12)
    .padding(24);

    for task in &app.scheduled_tasks {
        let tid = task.id.clone();
        list = list.push(
            row![
                column![
                    text(format!("{} ({})", task.title, task.cron_expression)).size(15),
                    text(format!(
                        "Status: {} · Next: {}",
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
        text("Messaging Gateway").size(20),
        text("Bridge Telegram, Discord, and Slack channels to your local Hermes runtime.").size(13),
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
                        "Status: {} · Profile: {}",
                        gw.connection_status, gw.linked_profile_id
                    ))
                    .size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
                button(text(if gw.enabled { "Enabled" } else { "Disabled" }).size(12))
                    .padding([6, 12])
                    .on_press(Message::ToggleGateway(gid, next_state)),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        );
    }

    scrollable(list).height(Length::Fill).into()
}
