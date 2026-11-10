//! Precision Native Dark Theme for Iced 0.13
//! Implements the 60-30-10 Deep Slate & Hermes Amber palette.

use iced::theme::{Custom, Palette};
use iced::{Color, Theme};

pub fn hermes_dark_theme() -> Theme {
    let palette = Palette {
        background: Color::from_rgb8(11, 15, 23), // #0B0F17 Deep Slate Canvas (60%)
        text: Color::from_rgb8(241, 245, 249),    // #F1F5F9 Crisp Slate-100
        primary: Color::from_rgb8(245, 158, 11),  // #F59E0B Hermes Amber Accent (10%)
        success: Color::from_rgb8(22, 163, 74),   // #16A34A Semantic Nominal
        danger: Color::from_rgb8(220, 38, 38),    // #DC2626 Semantic Alert
    };

    Theme::Custom(Box::new(Custom::new("Hermes Deep Slate".to_string(), palette)))
}
