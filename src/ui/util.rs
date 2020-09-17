use crate::api::model::*;
use crate::app::{ActiveBlock, App, SearchResultBlock};
use crate::config::app_config::Theme;
use tui::style::Style;

pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

pub fn get_color((is_active, is_hovered): (bool, bool), theme: Theme) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(theme.selected),
        (false, true) => Style::default().fg(theme.hovered),
        _ => Style::default().fg(theme.inactive),
    }
}

pub fn get_main_layout_margin(app: &App) -> u16 {
    if app.size.height > SMALL_TERMINAL_HEIGHT {
        1
    } else {
        0
    }
}
