use crossterm::cursor;
use crossterm::event::{Event, MouseEvent};
use crossterm::execute;
use crossterm::terminal;
use eyre::Result;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use std::io::{self, Write};
use std::panic;
use std::time::Duration;

use mal::auth::OAuth;

use mal::config::{AppConfig, AuthConfig};

// use mal::app::DebugInfo;

fn main() -> Result<()> {
    // get opts
    // let opts = cli::get_opts();

    // Get config

    // let mut app_config = AppConfig::load()?;

    let mut auth_config = AuthConfig::load()?;
    let mut oauth = OAuth::get_auth(&auth_config);

    Ok(())
}
