//! Hermes One Desktop — Exact 1:1 Dark Theme & Widget Styles for Iced 0.13
//! Replicates fathah/hermes-desktop [data-theme="dark"] tokens:
//! --bg-primary: #212121, --bg-secondary: #171717, --bg-tertiary: #2f2f2f,
//! --bg-hover: #3a3a3a, --accent: #003f7a, --text-primary: #ececec, --text-secondary: #b4b4b4

use iced::theme::Palette;
use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Theme};

pub const BG_PRIMARY: Color = Color::from_rgb(0.129, 0.129, 0.129); // #212121
pub const BG_SECONDARY: Color = Color::from_rgb(0.090, 0.090, 0.090); // #171717
pub const BG_TERTIARY: Color = Color::from_rgb(0.184, 0.184, 0.184); // #2f2f2f
pub const BG_CARD: Color = Color::from_rgb(0.165, 0.165, 0.165); // #2a2a2a
pub const BG_HOVER: Color = Color::from_rgb(0.227, 0.227, 0.227); // #3a3a3a
pub const ACCENT: Color = Color::from_rgb(0.0, 0.247, 0.478); // #003f7a
pub const ACCENT_HOVER: Color = Color::from_rgb(0.0, 0.333, 0.643); // #0055a4
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.925, 0.925, 0.925); // #ececec
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.706, 0.706, 0.706); // #b4b4b4
pub const TEXT_MUTED: Color = Color::from_rgb(0.557, 0.557, 0.557); // #8e8e8e
pub const BORDER_SUBTLE: Color = Color::from_rgb(0.165, 0.165, 0.165); // #2a2a2a
pub const BORDER_BRIGHT: Color = Color::from_rgb(0.235, 0.235, 0.235); // #3c3c3c
pub const GREEN_ONLINE: Color = Color::from_rgb(0.133, 0.773, 0.369); // #22c55e

pub fn hermes_dark_theme() -> Theme {
    Theme::custom(
        "Hermes Dark".to_string(),
        Palette {
            background: BG_PRIMARY,
            text: TEXT_PRIMARY,
            primary: ACCENT,
            success: GREEN_ONLINE,
            danger: Color::from_rgb8(0xEF, 0x44, 0x44),
        },
    )
}

/* ----- Container Styles ----- */

pub fn sidebar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_SECONDARY)),
        text_color: Some(TEXT_PRIMARY),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn main_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_PRIMARY)),
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn sessions_bar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_SECONDARY)),
        text_color: Some(TEXT_SECONDARY),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn status_bar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_SECONDARY)),
        text_color: Some(TEXT_MUTED),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn composer_shell_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_TERTIARY)),
        text_color: Some(TEXT_PRIMARY),
        border: Border {
            color: BORDER_BRIGHT,
            width: 1.0,
            radius: 14.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_CARD)),
        text_color: Some(TEXT_PRIMARY),
        border: Border {
            color: BORDER_BRIGHT,
            width: 1.0,
            radius: 10.0.into(),
        },
        ..Default::default()
    }
}

pub fn user_bubble_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(ACCENT)),
        text_color: Some(Color::WHITE),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 14.0.into(),
        },
        ..Default::default()
    }
}

pub fn agent_bubble_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_TERTIARY)),
        text_color: Some(TEXT_PRIMARY),
        border: Border {
            color: BORDER_BRIGHT,
            width: 1.0,
            radius: 14.0.into(),
        },
        ..Default::default()
    }
}

pub fn online_dot_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(GREEN_ONLINE)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn avatar_badge_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(ACCENT)),
        text_color: Some(Color::WHITE),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 11.0.into(),
        },
        ..Default::default()
    }
}

/* ----- Button Styles ----- */

pub fn sidebar_nav_button(is_active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme: &Theme, status: button::Status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
        let bg = if is_active {
            Some(Background::Color(BG_TERTIARY))
        } else if hovered {
            Some(Background::Color(Color::from_rgb8(0x25, 0x25, 0x25)))
        } else {
            Some(Background::Color(Color::TRANSPARENT))
        };
        let txt = if is_active || hovered {
            TEXT_PRIMARY
        } else {
            TEXT_SECONDARY
        };
        button::Style {
            background: bg,
            text_color: txt,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn profile_switcher_button(_theme: &Theme, status: button::Status) -> button::Style {
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
    button::Style {
        background: Some(Background::Color(if hovered { BG_TERTIARY } else { BG_PRIMARY })),
        text_color: TEXT_PRIMARY,
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn session_tab_button(is_active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme: &Theme, status: button::Status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
        let bg = if is_active {
            Some(Background::Color(BG_PRIMARY))
        } else if hovered {
            Some(Background::Color(Color::from_rgb8(0x23, 0x23, 0x23)))
        } else {
            Some(Background::Color(Color::TRANSPARENT))
        };
        button::Style {
            background: bg,
            text_color: if is_active { TEXT_PRIMARY } else { TEXT_SECONDARY },
            border: Border {
                color: if is_active { BORDER_BRIGHT } else { Color::TRANSPARENT },
                width: if is_active { 1.0 } else { 0.0 },
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn suggestion_pill_button(_theme: &Theme, status: button::Status) -> button::Style {
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
    button::Style {
        background: Some(Background::Color(if hovered { BG_HOVER } else { BG_CARD })),
        text_color: if hovered { TEXT_PRIMARY } else { TEXT_SECONDARY },
        border: Border {
            color: BORDER_BRIGHT,
            width: 1.0,
            radius: 18.0.into(),
        },
        ..Default::default()
    }
}

pub fn toolbar_chip_button(_theme: &Theme, status: button::Status) -> button::Style {
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
    button::Style {
        background: Some(Background::Color(if hovered {
            BG_HOVER
        } else {
            Color::from_rgb8(0x26, 0x26, 0x26)
        })),
        text_color: TEXT_SECONDARY,
        border: Border {
            color: BORDER_BRIGHT,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

pub fn send_circle_button(_theme: &Theme, status: button::Status) -> button::Style {
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
    button::Style {
        background: Some(Background::Color(if hovered { ACCENT_HOVER } else { ACCENT })),
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 15.0.into(),
        },
        ..Default::default()
    }
}

pub fn action_button(is_primary: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme: &Theme, status: button::Status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
        let bg = if is_primary {
            if hovered { ACCENT_HOVER } else { ACCENT }
        } else if hovered {
            BG_HOVER
        } else {
            BG_TERTIARY
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: TEXT_PRIMARY,
            border: Border {
                color: if is_primary { Color::TRANSPARENT } else { BORDER_BRIGHT },
                width: if is_primary { 0.0 } else { 1.0 },
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

/* ----- TextInput Style ----- */

pub fn composer_input_style(_theme: &Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Background::Color(BG_TERTIARY),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        icon: TEXT_MUTED,
        placeholder: TEXT_MUTED,
        value: TEXT_PRIMARY,
        selection: ACCENT,
    }
}

pub fn search_input_style(_theme: &Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Background::Color(BG_SECONDARY),
        border: Border {
            color: BORDER_BRIGHT,
            width: 1.0,
            radius: 8.0.into(),
        },
        icon: TEXT_MUTED,
        placeholder: TEXT_MUTED,
        value: TEXT_PRIMARY,
        selection: ACCENT,
    }
}
