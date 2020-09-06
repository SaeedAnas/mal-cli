use crate::api::model::*;
use crate::config::AppConfig;
use crate::network::IoEvent;
use std::sync::mpsc::Sender;

pub struct SearchResult {
    pub anime: Option<Page<Anime>>,
    pub manga: Option<Page<Manga>>,
    pub selected_anime_index: Option<usize>,
    pub selected_manga_index: Option<usize>,
    pub hovered_block: SearchResultBlock,
    pub selected_block: SearchResultBlock,
}

#[derive(PartialEq, Debug)]
pub enum SearchResultBlock {
    AnimeSearch,
    MangaSearch,
    Empty,
}

pub struct App {
    pub io_tx: Sender<IoEvent>,
    pub app_config: AppConfig,
    pub is_loading: bool,
    pub api_error: String,
    pub search_results: SearchResult,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>, app_config: AppConfig) -> Self {
        Self {
            io_tx,
            app_config,
            is_loading: false,
            api_error: String::new(),
            search_results: SearchResult {
                hovered_block: SearchResultBlock::AnimeSearch,
                selected_block: SearchResultBlock::Empty,
                anime: None,
                manga: None,
                selected_anime_index: None,
                selected_manga_index: None,
            },
        }
    }

    // TODO:  <05-09-20, yourname>  Handle errors in TUI //
    pub fn handle_error(&mut self) {
        self.api_error = "Error!".to_string();
    }
}
