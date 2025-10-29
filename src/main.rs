mod app;
mod error;
mod models;
mod services;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use app::state::AppState;
use services::connection::ConnectionService;
use services::query::QueryService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // appstate
    let mut state = AppState::new();
    // connecting to mongo
    let mut conn_service = ConnectionService::new();
    let uri = "mongodb://localhost:27017";

    match conn_service.connect(uri).await {
        Ok(server_info) => {
            state.set_connection(uri.to_string(), server_info);

            // load databases
            if let Some(client) = conn_service.get_client() {
                let query_service = QueryService::new(client.clone());
                match query_service.list_databases().await {
                    Ok(databases) => {
                        state.set_databases(databases);
                    }
                    Err(e) => {
                        state.set_error(Some(format!("Failed to load databases: {}", e)));
                    }
                }
            }
        }
        Err(e) => {
            state.set_error(Some(format!("Connection failed: {}", e)));
        }
    }

    // main loop of terminal
    // Main loop
    loop {
        terminal.draw(|f| match state.current_screen {
            app::screen::Screen::DatabaseList => {
                ui::database_list::render(f, f.area(), &state);
            }
            app::screen::Screen::CollectionList => {
                ui::collection_list::render(f, f.area(), &state);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match state.current_screen {
                app::screen::Screen::DatabaseList => {
                    match key.code {
                        KeyCode::Char('q') => {
                            state.quit();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            state.select_next_db();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            state.select_prev_db();
                        }
                        KeyCode::Enter => {
                            // Get database name first (clone to avoid borrow issues)
                            let db_name = state.get_selected_database().map(|db| db.name.clone());

                            if let Some(db_name) = db_name {
                                state.set_loading(true);
                                if let Some(client) = conn_service.get_client() {
                                    let query_service = QueryService::new(client.clone());
                                    match query_service.list_collections(&db_name).await {
                                        Ok(collections) => {
                                            state.current_database = Some(db_name);
                                            state.set_collections(collections);
                                            state.set_screen(app::screen::Screen::CollectionList);
                                        }
                                        Err(e) => {
                                            state.set_error(Some(format!(
                                                "Failed to load collections: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                state.set_loading(false);
                            }
                        }

                        _ => {}
                    }
                }
                app::screen::Screen::CollectionList => match key.code {
                    KeyCode::Char('q') => {
                        state.quit();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        state.select_next_coll();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        state.select_prev_coll();
                    }
                    KeyCode::Backspace => {
                        state.set_screen(app::screen::Screen::DatabaseList);
                    }
                    _ => {}
                },
            }
        }

        if state.should_quit {
            break;
        }
    }

    // restore terminal to its previous state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
