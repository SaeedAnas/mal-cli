mod anime;
mod common;
mod help;
mod input;
mod manga;
mod user;

use crate::api::model::*;
use crate::app::{ActiveBlock, App, RouteId, SearchResultBlock};
use crate::event::Key;
use crate::network::IoEvent;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        _ if key == app.app_config.keys.help => {
            app.set_current_route_state(Some(ActiveBlock::Help), None);
        }
        _ if key == app.app_config.keys.search => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        _ => handle_block_events(key, app),
    }
}

// Handler event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.get_current_route();
    match current_route.active_block {
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        ActiveBlock::SearchResultBlock => {}
        ActiveBlock::Empty => {}
        ActiveBlock::UserStats => {}
        ActiveBlock::Error => {}
        ActiveBlock::Help => {
            help::handler(key, app);
        }
        ActiveBlock::BasicView => {}
        ActiveBlock::Anime => {
            anime::handler(key, app);
        }
        ActiveBlock::Manga => {
            manga::handler(key, app);
        }
        ActiveBlock::User => {
            user::handler(key, app);
        }
    }
}

fn handle_escape(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::SearchResultBlock => {
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
        ActiveBlock::Error => {
            app.pop_navigation_stack();
        }
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
