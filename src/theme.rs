//! Hermes One Desktop — Exact 1:1 Dark Palette for Iced 0.13
//! Matches fathah/hermes-desktop [data-theme="dark"]:
//! --bg-primary: #212121, --bg-secondary: #171717, --accent: #003f7a, --text-primary: #ececec

use iced::theme::Palette;
use iced::{Color, Theme};

pub fn hermes_dark_theme() -> Theme {
    Theme::custom(
        "Hermes Dark".to_string(),
        Palette {
            background: Color::from_rgb8(0x21, 0x21, 0x21), // #212121
            text: Color::from_rgb8(0xEC, 0xEC, 0xEC),       // #ececec
            primary: Color::from_rgb8(0x00, 0x3F, 0x7A),    // #003f7a
            success: Color::from_rgb8(0x22, 0xC5, 0x5E),    // #22c55e
            danger: Color::from_rgb8(0xEF, 0x44, 0x44),     // #ef4444
        },
    )
}
