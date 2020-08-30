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

use clap::{App, Arg};

use mal::auth::OAuth;
use mal::config::{AppConfig, AuthConfig};
use mal::BANNER;

fn main() -> Result<()> {
    better_panic::install();

    setup_panic_hook();

    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .before_help(BANNER)
        .after_help(
            "\
        Your Config is stored in $HOME/.config/mal-cli/mal.yml\n\
        Your MAL Client ID is stored in $HOME/.config/mal-cli/oauth2.yml\
            ",
        )
        .arg(
            Arg::with_name("search")
                .short("s")
                .long("search")
                .value_name("INPUT")
                .help("Searches for anime/manga")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("seasonal")
                .short("S")
                .long("seasonal")
                .value_name("Season>,<Year")
                .default_value("now")
                .help("Shows Seasonal Anime")
                .takes_value(true),
        )
        .get_matches();

    println!("{:?}", args);

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    setup_terminal();

    // Get config
    let mut app_config = AppConfig::load()?;

    let mut auth_config = AuthConfig::load()?;
    let mut oauth = OAuth::get_auth(&auth_config);

    // initialize app state

    // start ui

    cleanup_terminal();
    Ok(())
}

fn setup_terminal() {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    execute!(stdout, crossterm::event::EnableMouseCapture).unwrap();

    terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
    let mut stdout = io::stdout();

    execute!(stdout, crossterm::event::DisableMouseCapture).unwrap();

    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();

    terminal::disable_raw_mode().unwrap();
}

fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        cleanup_terminal();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));
}
