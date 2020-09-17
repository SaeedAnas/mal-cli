use crate::event::Key;
use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug, Clone, Copy)]
/// Configuration for event handling.
pub struct EventConfig {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

/// An occurred event
pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wraps crossterm input and tick events.
/// Each Event is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    // Needs to be kept in order to prevent disposing of the sender side
    _tx: mpsc::Sender<Event<Key>>,
}

impl Events {
    pub fn new(tick_rate: u64) -> Self {
        Events::with_config(EventConfig {
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    /// Constructs a new instance of `Events` from a given config.
    pub fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(config.tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    let key = Key::from(key);

                    event_tx.send(Event::Input(key)).unwrap();
                }
            }

            event_tx.send(Event::Tick).unwrap();
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
