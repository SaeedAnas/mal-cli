use crossterm::cursor;
use crossterm::event::{Event, MouseEvent};
use crossterm::execute;
use crossterm::terminal;
use eyre::Result;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use std::io::{self, Write};
use std::panic;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use clap::{App as Clap, Arg};

use mal::app::App;
use mal::auth::OAuth;
use mal::config::{AppConfig, AuthConfig};
use mal::network::{IoEvent, Network};
use mal::BANNER;

fn setup_terminal() -> Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    execute!(stdout, crossterm::event::EnableMouseCapture)?;

    terminal::enable_raw_mode()?;
    Ok(())
}

fn cleanup_terminal() -> Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, crossterm::event::DisableMouseCapture)?;

    execute!(stdout, cursor::MoveTo(0, 0))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    execute!(stdout, cursor::Show)?;

    terminal::disable_raw_mode()?;
    Ok(())
}

fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        cleanup_terminal().unwrap();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));
}

#[tokio::main]
async fn main() -> Result<()> {
    better_panic::install();

    setup_panic_hook();

    // Set up clap app and get matches
    let args = Clap::new(env!("CARGO_PKG_NAME"))
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

    // Get config
    let app_config = AppConfig::load()?;

    let auth_config = AuthConfig::load()?;
    let oauth = OAuth::get_auth(auth_config)?;

    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

    // initialize app state
    let app = Arc::new(Mutex::new(App::new(sync_io_tx, app_config.clone())));

    let cloned_app = Arc::clone(&app);
    std::thread::spawn(move || {
        let mut network = Network::new(oauth, &app);
        start_network(sync_io_rx, &mut network);
    });

    // run ui
    start_ui(app_config, &cloned_app).await?;

    Ok(())
}

#[tokio::main]
async fn start_network<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}

async fn start_ui(app_config: AppConfig, app: &Arc<Mutex<App>>) -> Result<()> {
    // set up terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    setup_terminal()?;

    let mut is_first_render = true;

    loop {
        // let mut app = app.lock().await;
        // // TODO: Deal with resize events
        // let current_route = app.get_current_route();
        // terminal.draw(|mut f| match current_route.active_block {
        //     // TODO: Add active views

        // })

        // if current_route.active_block == ActiveBlock::Input {
        //     terminal.show_cursor()?;
        // } else {
        //     terminal.hide_cursor()?;
        // }
    }

    // clean up terminal
    cleanup_terminal()?;
    Ok(())
}
