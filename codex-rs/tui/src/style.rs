use crate::color::blend;
use crate::color::is_light;
use crate::terminal_palette::StdoutColorLevel;
use crate::terminal_palette::best_color;
use crate::terminal_palette::best_color_for_level;
use crate::terminal_palette::default_bg;
use crate::terminal_palette::stdout_color_level;
use ratatui::style::Color;
use ratatui::style::Style;

const USER_HISTORY_MESSAGE_ORANGE: (u8, u8, u8) = (255, 165, 0);

pub fn user_message_style() -> Style {
    user_message_style_for(default_bg())
}

pub fn user_history_message_style() -> Style {
    user_history_message_style_for(default_bg())
}

pub fn proposed_plan_style() -> Style {
    proposed_plan_style_for(default_bg())
}

/// Returns the style for a user-authored message using the provided terminal background.
pub fn user_message_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
    match terminal_bg {
        Some(bg) => Style::default().bg(user_message_bg(bg)),
        None => Style::default(),
    }
}

/// Returns the style for committed user history messages.
///
/// We keep the existing user-message background tint and choose the closest orange the terminal
/// can safely render. Low-color terminals fall back to ANSI red.
pub fn user_history_message_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
    user_message_style_for(terminal_bg).fg(user_history_message_fg())
}

pub fn proposed_plan_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
    match terminal_bg {
        Some(bg) => Style::default().bg(proposed_plan_bg(bg)),
        None => Style::default(),
    }
}

pub fn user_history_message_fg() -> Color {
    user_history_message_fg_for(stdout_color_level())
}

pub fn user_history_message_fg_for(color_level: StdoutColorLevel) -> Color {
    match color_level {
        StdoutColorLevel::TrueColor | StdoutColorLevel::Ansi256 => {
            best_color_for_level(USER_HISTORY_MESSAGE_ORANGE, color_level)
        }
        StdoutColorLevel::Ansi16 | StdoutColorLevel::Unknown => Color::Red,
    }
}

#[allow(clippy::disallowed_methods)]
pub fn user_message_bg(terminal_bg: (u8, u8, u8)) -> Color {
    let (top, alpha) = if is_light(terminal_bg) {
        ((0, 0, 0), 0.04)
    } else {
        ((255, 255, 255), 0.12)
    };
    best_color(blend(top, terminal_bg, alpha))
}

#[allow(clippy::disallowed_methods)]
pub fn proposed_plan_bg(terminal_bg: (u8, u8, u8)) -> Color {
    user_message_bg(terminal_bg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal_palette::indexed_color;
    use crate::terminal_palette::rgb_color;

    #[test]
    fn user_history_message_fg_truecolor_returns_orange() {
        assert_eq!(
            user_history_message_fg_for(StdoutColorLevel::TrueColor),
            rgb_color(USER_HISTORY_MESSAGE_ORANGE)
        );
    }

    #[test]
    fn user_history_message_fg_ansi256_returns_nearest_orange_index() {
        assert_eq!(
            user_history_message_fg_for(StdoutColorLevel::Ansi256),
            indexed_color(214)
        );
    }

    #[test]
    fn user_history_message_fg_low_color_falls_back_to_red() {
        assert_eq!(
            user_history_message_fg_for(StdoutColorLevel::Ansi16),
            Color::Red
        );
        assert_eq!(
            user_history_message_fg_for(StdoutColorLevel::Unknown),
            Color::Red
        );
    }
}
