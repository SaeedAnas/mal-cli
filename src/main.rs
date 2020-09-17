use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal;
use crossterm::{cursor::MoveTo, style::Print, ExecutableCommand};
use eyre::Result;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use std::io::{self, Write};
use std::panic;
use std::sync::Arc;
use tokio::sync::Mutex;

use clap::{App as ClapApp, Arg};

use mal::app::*;
use mal::auth::OAuth;
use mal::config::{AppConfig, AuthConfig};
use mal::event;
use mal::event::key::Key;
use mal::handlers;
use mal::network::{IoEvent, Network};
use mal::ui;
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
    let args = ClapApp::new(env!("CARGO_PKG_NAME"))
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
    println!("Here");

    // Get config
    let app_config = AppConfig::load()?;
    println!("Here");

    let auth_config = AuthConfig::load()?;
    println!("Here");
    let oauth = OAuth::get_auth_async(auth_config).await?;

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

    let events = event::Events::new(app_config.behavior.tick_rate_milliseconds);

    let mut is_first_render = true;

    loop {
        let mut app = app.lock().await;

        let current_route = app.get_current_route();
        terminal.draw(|mut f| match current_route.active_block {
            ActiveBlock::Help => {
                ui::draw_help_menu(&mut f, &app);
            }
            ActiveBlock::Error => {
                ui::draw_error(&mut f, &app);
            }
            _ => {
                ui::draw_main_layout(&mut f, &app);
            }
        })?;

        if current_route.active_block == ActiveBlock::Input {
            terminal.show_cursor()?;
        } else {
            terminal.hide_cursor()?;
        }

        let cursor_offset = if app.size.height > ui::util::SMALL_TERMINAL_HEIGHT {
            2
        } else {
            1
        };

        terminal.backend_mut().execute(MoveTo(
            cursor_offset + app.input_cursor_position,
            cursor_offset,
        ))?;

        match events.next()? {
            event::Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }

                let current_active_block = app.get_current_route().active_block;

                if current_active_block == ActiveBlock::Input {
                    handlers::input_handler(key, &mut app);
                } else if key == app.app_config.keys.back {
                    if app.get_current_route().active_block != ActiveBlock::Input {
                        let pop_result = match app.pop_navigation_stack() {
                            Some(ref x) if x.id == RouteId::Search => app.pop_navigation_stack(),
                            Some(x) => Some(x),
                            None => None,
                        };
                        if pop_result.is_none() {
                            break;
                        }
                    }
                } else {
                    handlers::handle_app(key, &mut app);
                }
            }
            _ => {}
        }
    }

    // clean up terminal
    cleanup_terminal()?;
    Ok(())
}
