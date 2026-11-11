//! Native GPU-rendered Iced 0.13 views replicating fathah/hermes-desktop 1:1
//! Uses custom dark surfaces (#212121 / #171717 / #2f2f2f), ghost sidebar items,
//! rounded suggestion pills, unified chat composer card, and embedded vector SVG icons.

use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, vertical_space,
};
use iced::{Alignment, Element, Length};

use crate::HermesApp;
use crate::icons;
use crate::message::{ActiveTab, Message};
use crate::models::TaskStatus;
use crate::theme;

pub fn view_sidebar(app: &HermesApp) -> Element<'_, Message> {
    let new_chat_btn = button(
        row![
            icons::icon(icons::ICON_PLUS, 15.0),
            text("New Chat").size(13),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([8, 10])
    .style(theme::sidebar_nav_button(false))
    .on_press(Message::CreateSession);

    let nav_btn = |icon_bytes: &'static [u8], label: &'static str, tab: ActiveTab| {
        let is_active = app.active_tab == tab;
        button(
            row![
                icons::icon(icon_bytes, 15.0),
                text(label).size(13),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([8, 10])
        .style(theme::sidebar_nav_button(is_active))
        .on_press(Message::SwitchTab(tab))
    };

    let pinned_nav = column![
        new_chat_btn,
        nav_btn(icons::ICON_COMPASS, "Discover", ActiveTab::Discover),
        nav_btn(icons::ICON_BUILDING, "Office", ActiveTab::Office),
        nav_btn(icons::ICON_KANBAN, "Kanban", ActiveTab::Kanban),
        nav_btn(icons::ICON_TIMER, "Schedules", ActiveTab::Schedules),
    ]
    .spacing(2);

    let mut recent_chats = column![
        container(text("Chats").size(11).color(theme::TEXT_MUTED)).padding([10, 10]),
    ]
    .spacing(2);

    for sess in &app.sessions {
        let is_current = app.active_tab == ActiveTab::Chat && app.active_session_id == sess.id;
        recent_chats = recent_chats.push(
            row![
                button(text(&sess.title).size(12.5))
                    .width(Length::Fill)
                    .padding([6, 10])
                    .style(theme::sidebar_nav_button(is_current))
                    .on_press(Message::SelectSession(sess.id.clone())),
                button(icons::icon(icons::ICON_CLOSE, 11.0))
                    .padding([6, 6])
                    .style(theme::sidebar_nav_button(false))
                    .on_press(Message::DeleteSession(sess.id.clone())),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        );
    }

    let footer_icon_btn = |icon_bytes: &'static [u8], tab: ActiveTab| {
        let is_active = app.active_tab == tab;
        button(icons::icon(icon_bytes, 15.0))
            .padding([7, 9])
            .style(theme::sidebar_nav_button(is_active))
            .on_press(Message::SwitchTab(tab))
    };

    let footer_actions = row![
        footer_icon_btn(icons::ICON_KEY, ActiveTab::Providers),
        footer_icon_btn(icons::ICON_SIGNAL, ActiveTab::Gateways),
        footer_icon_btn(icons::ICON_WORKFLOW, ActiveTab::Toolsets),
        footer_icon_btn(icons::ICON_BRAIN, ActiveTab::Memory),
        footer_icon_btn(icons::ICON_SETTINGS, ActiveTab::Profiles),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let active_profile_name = app
        .profiles
        .iter()
        .find(|p| p.id == app.active_profile_id)
        .map(|p| p.name.as_str())
        .unwrap_or("default");

    let avatar_letter = active_profile_name
        .chars()
        .next()
        .unwrap_or('D')
        .to_uppercase()
        .to_string();

    let avatar_badge = container(text(avatar_letter).size(11))
        .width(Length::Fixed(22.0))
        .height(Length::Fixed(22.0))
        .center_x(Length::Fixed(22.0))
        .center_y(Length::Fixed(22.0))
        .style(theme::avatar_badge_container);

    let profile_switcher = button(
        row![
            avatar_badge,
            text(active_profile_name).size(12.5),
            horizontal_space(),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([6, 8])
    .style(theme::profile_switcher_button)
    .on_press(Message::SwitchTab(ActiveTab::Profiles));

    let sidebar_col = column![
        pinned_nav,
        scrollable(recent_chats).height(Length::Fill),
        footer_actions,
        profile_switcher,
    ]
    .spacing(8)
    .padding(10)
    .width(Length::Fixed(220.0));

    container(sidebar_col)
        .height(Length::Fill)
        .style(theme::sidebar_container)
        .into()
}

pub fn view_active_sessions_bar(app: &HermesApp) -> Element<'_, Message> {
    let mut tabs = row![].spacing(4).align_y(Alignment::Center);
    for sess in app.sessions.iter().take(6) {
        let is_active = app.active_tab == ActiveTab::Chat && app.active_session_id == sess.id;
        tabs = tabs.push(
            button(
                row![
                    text(&sess.title).size(12),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([5, 11])
            .style(theme::session_tab_button(is_active))
            .on_press(Message::SelectSession(sess.id.clone())),
        );
    }
    tabs = tabs.push(
        button(icons::icon(icons::ICON_PLUS, 13.0))
            .padding([6, 8])
            .style(theme::sidebar_nav_button(false))
            .on_press(Message::CreateSession),
    );

    container(tabs)
        .width(Length::Fill)
        .padding([6, 12])
        .style(theme::sessions_bar_container)
        .into()
}

pub fn view_status_bar(app: &HermesApp) -> Element<'_, Message> {
    let active_model = app
        .profiles
        .iter()
        .find(|p| p.id == app.active_profile_id)
        .map(|p| {
            p.model_name
                .rsplit('/')
                .next()
                .unwrap_or(p.model_name.as_str())
        })
        .unwrap_or("hermes-3-llama-3.1-405b");

    let enabled_skills = app.toolsets.iter().filter(|t| t.enabled).count();

    let green_dot = container(text(""))
        .width(Length::Fixed(7.0))
        .height(Length::Fixed(7.0))
        .style(theme::online_dot_container);

    let left_group = row![
        green_dot,
        text("gateway").size(11).color(theme::TEXT_SECONDARY),
        text("·").size(11).color(theme::TEXT_MUTED),
        text("local").size(11).color(theme::TEXT_MUTED),
        text("·").size(11).color(theme::TEXT_MUTED),
        text(active_model).size(11).color(theme::TEXT_MUTED),
        text("·").size(11).color(theme::TEXT_MUTED),
        text(format!("{} skills", enabled_skills))
            .size(11)
            .color(theme::TEXT_MUTED),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let right_group = row![
        text("/ commands").size(11).color(theme::TEXT_MUTED),
        text("·").size(11).color(theme::TEXT_MUTED),
        text("Ctrl, settings").size(11).color(theme::TEXT_MUTED),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let bar = row![left_group, horizontal_space(), right_group]
        .align_y(Alignment::Center)
        .padding([5, 14]);

    container(bar)
        .width(Length::Fill)
        .style(theme::status_bar_container)
        .into()
}

pub fn view_chat(app: &HermesApp) -> Element<'_, Message> {
    let body: Element<'_, Message> = if app.messages.is_empty() && !app.is_streaming {
        let suggestion_btn =
            |icon_bytes: &'static [u8], label: &'static str, prompt: &'static str| {
                button(
                    row![
                        icons::icon(icon_bytes, 13.0),
                        text(label).size(12.5),
                    ]
                    .spacing(7)
                    .align_y(Alignment::Center),
                )
                .padding([7, 14])
                .style(theme::suggestion_pill_button)
                .on_press(Message::PrefillSuggestion(prompt.to_string()))
            };

        let suggestions_row1 = row![
            suggestion_btn(
                icons::ICON_GLOBE,
                "Search the web",
                "Search the web for the latest developments in "
            ),
            suggestion_btn(
                icons::ICON_BELL,
                "Set a reminder",
                "Remind me in 30 minutes to "
            ),
            suggestion_btn(
                icons::ICON_MAIL,
                "Summarize emails",
                "Check my recent emails and summarize anything important"
            ),
        ]
        .spacing(8);

        let suggestions_row2 = row![
            suggestion_btn(
                icons::ICON_TERMINAL,
                "Write a script",
                "Write a Python script that "
            ),
            suggestion_btn(
                icons::ICON_CLOCK,
                "Schedule a cron job",
                "Set up a daily scheduled task at 9am to "
            ),
            suggestion_btn(
                icons::ICON_CHART,
                "Analyze data",
                "Help me analyze and visualize "
            ),
        ]
        .spacing(8);

        let empty_col = column![
            vertical_space(),
            icons::hermes_logo(),
            text("How can I help you today?")
                .size(21)
                .color(theme::TEXT_PRIMARY),
            text("Ask me to write code, answer questions, search the web, and more")
                .size(13)
                .color(theme::TEXT_MUTED),
            container(column![suggestions_row1, suggestions_row2].spacing(8).align_x(Alignment::Center))
                .padding([10, 0]),
            vertical_space(),
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .width(Length::Fill);

        container(empty_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    } else {
        let mut msg_list = column![]
            .spacing(12)
            .padding([20, 48])
            .max_width(820.0);

        for msg in &app.messages {
            let is_user = msg.role == "user";
            let bubble = container(
                column![
                    text(&msg.content).size(13.5),
                    text(&msg.created_at).size(10.5).color(if is_user {
                        theme::TEXT_SECONDARY
                    } else {
                        theme::TEXT_MUTED
                    }),
                ]
                .spacing(4),
            )
            .padding([10, 14])
            .max_width(620.0)
            .style(if is_user {
                theme::user_bubble_container
            } else {
                theme::agent_bubble_container
            });

            if is_user {
                msg_list = msg_list.push(row![horizontal_space(), bubble]);
            } else {
                msg_list = msg_list.push(row![bubble, horizontal_space()]);
            }
        }

        if app.is_streaming {
            let stream_text = if app.streaming_buffer.is_empty() {
                "Thinking..."
            } else {
                app.streaming_buffer.as_str()
            };
            let bubble = container(text(stream_text).size(13.5))
                .padding([10, 14])
                .max_width(620.0)
                .style(theme::agent_bubble_container);
            msg_list = msg_list.push(row![bubble, horizontal_space()]);
        }

        scrollable(
            container(msg_list)
                .width(Length::Fill)
                .center_x(Length::Fill),
        )
        .height(Length::Fill)
        .into()
    };

    let active_model = app
        .profiles
        .iter()
        .find(|p| p.id == app.active_profile_id)
        .map(|p| {
            p.model_name
                .rsplit('/')
                .next()
                .unwrap_or(p.model_name.as_str())
        })
        .unwrap_or("hermes-3-llama-3.1-405b");

    // Unified ChatInput Shell (.chat-input-shell -> .chat-input-wrapper)
    let input_box = text_input("Ask anything", &app.prompt_input)
        .on_input(Message::PromptInputChanged)
        .on_submit(Message::SubmitPrompt)
        .padding([8, 6])
        .size(14)
        .style(theme::composer_input_style);

    let toolbar_row = row![
        button(icons::icon(icons::ICON_PAPERCLIP, 14.0))
            .padding([5, 7])
            .style(theme::sidebar_nav_button(false))
            .on_press(Message::PrefillSuggestion("[attached: context.md] ".to_string())),
        button(icons::icon(icons::ICON_MIC, 14.0))
            .padding([5, 7])
            .style(theme::sidebar_nav_button(false)),
        button(text(active_model).size(11.5))
            .padding([4, 9])
            .style(theme::toolbar_chip_button)
            .on_press(Message::SwitchTab(ActiveTab::Providers)),
        button(
            row![
                icons::icon(icons::ICON_BRAIN, 12.0),
                text("medium").size(11.5),
            ]
            .spacing(5)
            .align_y(Alignment::Center)
        )
        .padding([4, 9])
        .style(theme::toolbar_chip_button),
        horizontal_space(),
        button(icons::icon(icons::ICON_ARROW_UP, 14.0))
            .width(Length::Fixed(30.0))
            .height(Length::Fixed(30.0))
            .padding(7)
            .style(theme::send_circle_button)
            .on_press(Message::SubmitPrompt),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let composer_card = container(column![input_box, toolbar_row].spacing(6))
        .width(Length::Fill)
        .max_width(760.0)
        .padding([10, 12])
        .style(theme::composer_shell_container);

    let composer_area = container(composer_card)
        .width(Length::Fill)
        .center_x(Length::Fill)
        .padding([8, 24]);

    column![body, composer_area]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn view_discover(app: &HermesApp) -> Element<'_, Message> {
    view_toolsets(app)
}

pub fn view_office(app: &HermesApp) -> Element<'_, Message> {
    let mut col = column![
        text("Hermes Agent Office").size(20).color(theme::TEXT_PRIMARY),
        text("Live multi-agent station status across Coordinator, Research, Code Forge, and Memory Vault.")
            .size(13)
            .color(theme::TEXT_MUTED),
    ]
    .spacing(12)
    .padding(28)
    .max_width(920.0);

    for prof in &app.profiles {
        let is_active = prof.id == app.active_profile_id;
        col = col.push(
            container(
                row![
                    column![
                        text(format!("Station · {}", prof.name))
                            .size(14.5)
                            .color(theme::TEXT_PRIMARY),
                        text(format!("Model: {} · Dir: {}", prof.model_name, prof.hermes_dir))
                            .size(12)
                            .color(theme::TEXT_MUTED),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    button(text(if is_active { "Active Agent" } else { "Switch Agent" }).size(12))
                        .padding([6, 12])
                        .style(theme::action_button(is_active))
                        .on_press(Message::SelectProfile(prof.id.clone())),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .style(theme::card_container),
        );
    }

    scrollable(col).height(Length::Fill).into()
}

pub fn view_kanban(app: &HermesApp) -> Element<'_, Message> {
    view_tasks(app)
}

pub fn view_profiles(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Profiles & Providers").size(20).color(theme::TEXT_PRIMARY),
        text("Manage isolated ~/.hermes/profiles directories, API endpoints, and default models.")
            .size(13)
            .color(theme::TEXT_MUTED),
    ]
    .spacing(12)
    .padding(28)
    .max_width(920.0);

    for profile in &app.profiles {
        let is_selected = profile.id == app.active_profile_id;
        list = list.push(
            container(
                row![
                    column![
                        text(format!("{} ({})", profile.name, profile.provider.as_str()))
                            .size(14.5)
                            .color(theme::TEXT_PRIMARY),
                        text(format!(
                            "{} · {} · Context: {} tokens",
                            profile.model_name, profile.hermes_dir, profile.context_window
                        ))
                        .size(12)
                        .color(theme::TEXT_MUTED),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    button(text(if is_selected { "Active" } else { "Switch" }).size(12))
                        .padding([6, 14])
                        .style(theme::action_button(is_selected))
                        .on_press(Message::SelectProfile(profile.id.clone())),
                ]
                .spacing(16)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .style(theme::card_container),
        );
    }

    scrollable(list).height(Length::Fill).into()
}

pub fn view_memory(app: &HermesApp) -> Element<'_, Message> {
    let header = column![
        text("Memory & Identity (MEMORY.md / SOUL.md)")
            .size(20)
            .color(theme::TEXT_PRIMARY),
        row![
            text_input("Filter memories...", &app.memory_search_query)
                .on_input(Message::MemorySearchChanged)
                .padding([8, 10])
                .size(13)
                .style(theme::search_input_style),
            text_input("Add memory entry...", &app.new_memory_input)
                .on_input(Message::NewMemoryInputChanged)
                .padding([8, 10])
                .size(13)
                .style(theme::search_input_style),
            button(text("Add Memory").size(12.5))
                .padding([8, 14])
                .style(theme::action_button(true))
                .on_press(Message::AddMemoryEntry),
        ]
        .spacing(10),
    ]
    .spacing(12);

    let mut items = column![header].spacing(10).padding(28).max_width(920.0);
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
            container(
                row![
                    column![
                        text(format!("{} · {}", mem.category.as_str(), mem.created_at))
                            .size(11)
                            .color(theme::TEXT_MUTED),
                        text(&mem.content).size(13.5).color(theme::TEXT_PRIMARY),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    button(text("Delete").size(12))
                        .padding([6, 10])
                        .style(theme::action_button(false))
                        .on_press(Message::DeleteMemoryEntry(mem.id.clone())),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .style(theme::card_container),
        );
    }

    scrollable(items).height(Length::Fill).into()
}

pub fn view_toolsets(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Capabilities · Toolsets, Skills & MCP Servers")
            .size(20)
            .color(theme::TEXT_PRIMARY),
        text("Enable or disable built-in capabilities and ~/.hermes/plugins.")
            .size(13)
            .color(theme::TEXT_MUTED),
    ]
    .spacing(12)
    .padding(28)
    .max_width(920.0);

    for tool in &app.toolsets {
        let tid = tool.id.clone();
        let next_state = !tool.enabled;
        list = list.push(
            container(
                row![
                    column![
                        text(format!("{} · {}", tool.name, tool.category))
                            .size(14.5)
                            .color(theme::TEXT_PRIMARY),
                        text(format!("{} ({})", tool.description, tool.plugin_path))
                            .size(12)
                            .color(theme::TEXT_MUTED),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    button(text(if tool.enabled { "Enabled" } else { "Disabled" }).size(12))
                        .padding([6, 12])
                        .style(theme::action_button(tool.enabled))
                        .on_press(Message::ToggleToolset(tid, next_state)),
                ]
                .spacing(16)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .style(theme::card_container),
        );
    }

    scrollable(list).height(Length::Fill).into()
}

pub fn view_tasks(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Schedules & Cron Automation")
            .size(20)
            .color(theme::TEXT_PRIMARY),
        text("Automate recurring prompts and background agent tasks.")
            .size(13)
            .color(theme::TEXT_MUTED),
    ]
    .spacing(12)
    .padding(28)
    .max_width(920.0);

    for task in &app.scheduled_tasks {
        let tid = task.id.clone();
        list = list.push(
            container(
                row![
                    column![
                        text(format!("{} ({})", task.title, task.cron_expression))
                            .size(14.5)
                            .color(theme::TEXT_PRIMARY),
                        text(format!(
                            "Status: {} · Next: {}",
                            task.status.as_str(),
                            task.next_run_at
                        ))
                        .size(12)
                        .color(theme::TEXT_MUTED),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    button(text("Run Now").size(12))
                        .padding([6, 12])
                        .style(theme::action_button(true))
                        .on_press(Message::AdvanceTaskStatus(tid, TaskStatus::Completed)),
                ]
                .spacing(16)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .style(theme::card_container),
        );
    }

    scrollable(list).height(Length::Fill).into()
}

pub fn view_gateways(app: &HermesApp) -> Element<'_, Message> {
    let mut list = column![
        text("Messaging Gateway")
            .size(20)
            .color(theme::TEXT_PRIMARY),
        text("Bridge Telegram, Discord, and Slack channels to your local Hermes runtime.")
            .size(13)
            .color(theme::TEXT_MUTED),
    ]
    .spacing(12)
    .padding(28)
    .max_width(920.0);

    for gw in &app.gateways {
        let gid = gw.id.clone();
        let next_state = !gw.enabled;
        list = list.push(
            container(
                row![
                    column![
                        text(format!("{} · {}", gw.platform, gw.bot_handle))
                            .size(14.5)
                            .color(theme::TEXT_PRIMARY),
                        text(format!(
                            "Status: {} · Profile: {}",
                            gw.connection_status, gw.linked_profile_id
                        ))
                        .size(12)
                        .color(theme::TEXT_MUTED),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    button(text(if gw.enabled { "Enabled" } else { "Disabled" }).size(12))
                        .padding([6, 12])
                        .style(theme::action_button(gw.enabled))
                        .on_press(Message::ToggleGateway(gid, next_state)),
                ]
                .spacing(16)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .style(theme::card_container),
        );
    }

    scrollable(list).height(Length::Fill).into()
}
