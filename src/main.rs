mod app;
mod error;
mod models;
mod services;
mod ui;

use arboard::Clipboard;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
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
    // let uri = "mongodb://localhost:27017";

    // match conn_service.connect(uri).await {
    //     Ok(server_info) => {
    //         state.set_connection(uri.to_string(), server_info);

    //         // load databases
    //         if let Some(client) = conn_service.get_client() {
    //             let query_service = QueryService::new(client.clone());
    //             match query_service.list_databases().await {
    //                 Ok(databases) => {
    //                     state.set_databases(databases);
    //                 }
    //                 Err(e) => {
    //                     state.set_error(Some(format!("Failed to load databases: {}", e)));
    //                 }
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         state.set_error(Some(format!("Connection failed: {}", e)));
    //     }
    // }

    // main loop
    loop {
        terminal.draw(|f| match state.current_screen {
            app::screen::Screen::DatabaseList => {
                ui::database_list::render(f, f.area(), &state);
            }
            app::screen::Screen::Connection => {
                ui::connection::render(f, f.area(), &state);
            }
            app::screen::Screen::CollectionList => {
                ui::collection_list::render(f, f.area(), &state);
            }
            app::screen::Screen::DocumentView => {
                ui::document_view::render(f, f.area(), &state);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match state.current_screen {
                app::screen::Screen::Connection => match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.quit();
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.quit();
                    }
                    KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // pasting from clipboard
                        if let Ok(mut clipboard) = Clipboard::new() {
                            if let Ok(text) = clipboard.get_text() {
                                for c in text.chars() {
                                    state.push_char(c);
                                }
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        state.push_char(c);
                    }
                    KeyCode::Backspace => {
                        state.pop_char();
                    }
                    KeyCode::Esc => {
                        state.clear_input();
                        state.set_error(None);
                    }
                    KeyCode::Enter => {
                        let uri = state.connection_input.clone();
                        state.set_loading(true);
                        state.set_error(None);

                        match conn_service.connect(&uri).await {
                            Ok(server_info) => {
                                state.set_connection(uri, server_info);

                                if let Some(client) = conn_service.get_client() {
                                    let query_service = QueryService::new(client.clone());
                                    match query_service.list_databases().await {
                                        Ok(databases) => {
                                            state.set_databases(databases);
                                            state.set_screen(app::screen::Screen::DatabaseList);
                                        }
                                        Err(e) => {
                                            state.set_error(Some(format!(
                                                "Failed to load databases: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                state.set_error(Some(format!("Connection failed: {}", e)));
                            }
                        }
                        state.set_loading(false);
                    }
                    _ => {}
                },
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
                        KeyCode::Char('r') => {
                            // Refresh databases
                            state.set_loading(true);
                            if let Some(client) = conn_service.get_client() {
                                let query_service = QueryService::new(client.clone());
                                match query_service.list_databases().await {
                                    Ok(databases) => {
                                        state.set_databases(databases);
                                    }
                                    Err(e) => {
                                        state.set_error(Some(format!(
                                            "Failed to refresh databases: {}",
                                            e
                                        )));
                                    }
                                }
                            }
                            state.set_loading(false);
                        }

                        _ => {}
                    }
                }
                app::screen::Screen::CollectionList => {
                    match key.code {
                        KeyCode::Char('q') => {
                            state.quit();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            state.select_next_coll();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            state.select_prev_coll();
                        }
                        KeyCode::Enter => {
                            // Load documents for selected collection
                            let coll_name = state.get_selected_collection().map(|c| c.name.clone());
                            let db_name = state.current_database.clone();

                            if let (Some(db_name), Some(coll_name)) = (db_name, coll_name) {
                                state.set_loading(true);
                                if let Some(client) = conn_service.get_client() {
                                    let query_service = QueryService::new(client.clone());
                                    match query_service
                                        .find_documents(&db_name, &coll_name, None, 0, 20)
                                        .await
                                    {
                                        Ok(documents) => {
                                            state.current_collection = Some(coll_name);
                                            state.set_documents(documents);
                                            state.set_screen(app::screen::Screen::DocumentView);
                                        }
                                        Err(e) => {
                                            state.set_error(Some(format!(
                                                "Failed to load documents: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                state.set_loading(false);
                            }
                        }
                        KeyCode::Backspace => {
                            state.set_screen(app::screen::Screen::DatabaseList);
                        }
                        KeyCode::Char('r') => {
                            // Refresh collections
                            if let Some(db_name) = state.current_database.clone() {
                                state.set_loading(true);
                                if let Some(client) = conn_service.get_client() {
                                    let query_service = QueryService::new(client.clone());
                                    match query_service.list_collections(&db_name).await {
                                        Ok(collections) => {
                                            state.set_collections(collections);
                                        }
                                        Err(e) => {
                                            state.set_error(Some(format!(
                                                "Failed to refresh collections: {}",
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
                app::screen::Screen::DocumentView => {
                    if state.filter_mode {
                        // Filter input mode
                        match key.code {
                            KeyCode::Char(c) => {
                                state.push_filter_char(c);
                            }
                            KeyCode::Backspace => {
                                state.pop_filter_char();
                            }
                            KeyCode::Esc => {
                                state.exit_filter_mode();
                            }
                            KeyCode::Enter => {
                                match state.apply_filter() {
                                    Ok(_) => {
                                        state.exit_filter_mode();
                                        // Reload documents with filter
                                        let db_name = state.current_database.clone();
                                        let coll_name = state.current_collection.clone();
                                        let filter = state.filter.clone();

                                        if let (Some(db_name), Some(coll_name)) =
                                            (db_name, coll_name)
                                        {
                                            state.set_loading(true);
                                            if let Some(client) = conn_service.get_client() {
                                                let query_service =
                                                    QueryService::new(client.clone());
                                                match query_service
                                                    .find_documents(
                                                        &db_name, &coll_name, filter, 0, 20,
                                                    )
                                                    .await
                                                {
                                                    Ok(documents) => {
                                                        state.set_documents(documents);
                                                    }
                                                    Err(e) => {
                                                        state.set_error(Some(format!(
                                                            "Failed to apply filter: {}",
                                                            e
                                                        )));
                                                    }
                                                }
                                            }
                                            state.set_loading(false);
                                        }
                                    }
                                    Err(e) => {
                                        state.set_error(Some(e));
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        // Normal navigation mode
                        match key.code {
                            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                state.quit();
                            }
                            KeyCode::Char('f') => {
                                state.enter_filter_mode();
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                state.select_next_doc();
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                state.select_prev_doc();
                            }
                            KeyCode::PageDown => {
                                state.scroll_doc_down();
                            }
                            KeyCode::PageUp => {
                                state.scroll_doc_up();
                            }
                            KeyCode::Backspace => {
                                state.set_screen(app::screen::Screen::CollectionList);
                            }
                            KeyCode::Esc => {
                                state.clear_filter();
                                // Reload without filter
                                let db_name = state.current_database.clone();
                                let coll_name = state.current_collection.clone();

                                if let (Some(db_name), Some(coll_name)) = (db_name, coll_name) {
                                    state.set_loading(true);
                                    if let Some(client) = conn_service.get_client() {
                                        let query_service = QueryService::new(client.clone());
                                        match query_service
                                            .find_documents(&db_name, &coll_name, None, 0, 20)
                                            .await
                                        {
                                            Ok(documents) => {
                                                state.set_documents(documents);
                                            }
                                            Err(e) => {
                                                state.set_error(Some(format!(
                                                    "Failed to reload documents: {}",
                                                    e
                                                )));
                                            }
                                        }
                                    }
                                    state.set_loading(false);
                                }
                            }
                            KeyCode::Char('r') => {
                                // Refresh with current filter
                                let db_name = state.current_database.clone();
                                let coll_name = state.current_collection.clone();
                                let filter = state.filter.clone();

                                if let (Some(db_name), Some(coll_name)) = (db_name, coll_name) {
                                    state.set_loading(true);
                                    if let Some(client) = conn_service.get_client() {
                                        let query_service = QueryService::new(client.clone());
                                        match query_service
                                            .find_documents(&db_name, &coll_name, filter, 0, 20)
                                            .await
                                        {
                                            Ok(documents) => {
                                                state.set_documents(documents);
                                            }
                                            Err(e) => {
                                                state.set_error(Some(format!(
                                                    "Failed to refresh documents: {}",
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
                }
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
