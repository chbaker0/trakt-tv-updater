/// Application.
mod app;

/// Terminal events handler.
mod event;

/// Widget renderer.
mod ui;

/// Terminal user interface.
mod tui;

/// Event handler.
mod handler;

use crate::{
    interface::{
        app::{App, AppResult},
        event::{Event, EventHandler},
        handler::{handle_key_events, handle_mouse_events},
        tui::Tui,
    },
    models::TraktShow,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

pub fn run(items: Vec<TraktShow>) -> AppResult<()> {
    // Create an application.
    let mut app = App::new(items);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
