mod app;
mod config;
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

use crate::config::ConnectionHistory;

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
    let mut conn_service = ConnectionService::new();
    let mut history = ConnectionHistory::load();
    state.set_connection_history(history.uris.clone());

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
                app::screen::Screen::Connection => {
                    if state.show_history {
                        match key.code {
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                state.quit();
                            }
                            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                state.quit();
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                state.select_prev_history();
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                state.select_next_history();
                            }
                            KeyCode::Enter => {
                                if let Some(uri) = state.get_selected_history_uri() {
                                    state.connection_input = uri.clone();
                                    state.toggle_history();
                                }
                            }
                            KeyCode::Tab => {
                                state.toggle_history();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                state.quit();
                            }
                            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                state.quit();
                            }
                            KeyCode::Tab => {
                                state.toggle_history();
                            }
                            KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
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
                                        // Save to history
                                        history.add_uri(uri.clone());
                                        let _ = history.save();
                                        state.set_connection_history(history.uris.clone());

                                        state.set_connection(uri, server_info);

                                        if let Some(client) = conn_service.get_client() {
                                            let query_service = QueryService::new(client.clone());
                                            match query_service.list_databases().await {
                                                Ok(databases) => {
                                                    state.set_databases(databases);
                                                    state.set_screen(
                                                        app::screen::Screen::DatabaseList,
                                                    );
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
                        }
                    }
                }
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
                    if state.query_mode {
                        // Advanced query mode - JSON input
                        match key.code {
                            KeyCode::Char(c) => {
                                state.push_query_char(c);
                            }
                            KeyCode::Backspace => {
                                state.pop_query_char();
                            }
                            KeyCode::Esc => {
                                state.exit_query_mode();
                                state.clear_query();
                            }
                            KeyCode::Enter => {
                                // Apply the JSON query
                                match state.apply_filter() {
                                    Ok(_) => {
                                        state.exit_query_mode();
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
                                                        state.set_error(None);
                                                    }
                                                    Err(e) => {
                                                        state.set_error(Some(format!(
                                                            "Query failed: {}",
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
                    } else if state.filter_mode {
                        // Simple search mode - live filtering
                        match key.code {
                            KeyCode::Char(c) => {
                                state.push_filter_char(c);
                                apply_dynamic_filter(&mut state, &conn_service).await;
                            }
                            KeyCode::Backspace => {
                                state.pop_filter_char();
                                apply_dynamic_filter(&mut state, &conn_service).await;
                            }
                            KeyCode::Esc => {
                                state.exit_filter_mode();
                                state.clear_filter();
                                reload_documents_without_filter(&mut state, &conn_service).await;
                            }
                            KeyCode::Enter => {
                                state.exit_filter_mode();
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
                            KeyCode::Char('/') => {
                                state.enter_query_mode();
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
                                reload_documents_without_filter(&mut state, &conn_service).await;
                            }
                            KeyCode::Char('r') => {
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
                                                    "Failed to refresh: {}",
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

    // helper function for dynamic filtering
    async fn apply_dynamic_filter(state: &mut AppState, conn_service: &ConnectionService) {
        if state.filter_input.is_empty() {
            reload_documents_without_filter(state, conn_service).await;
            return;
        }

        let db_name = state.current_database.clone();
        let coll_name = state.current_collection.clone();

        if let (Some(db_name), Some(coll_name)) = (db_name, coll_name) {
            if let Some(client) = conn_service.get_client() {
                let query_service = QueryService::new(client.clone());

                // Get a sample document to extract field names
                match query_service
                    .find_documents(&db_name, &coll_name, None, 0, 1)
                    .await
                {
                    Ok(sample_docs) => {
                        if let Some(sample_doc) = sample_docs.first() {
                            // Build $or array with regex for each field
                            let mut or_conditions = Vec::new();

                            for (key, _) in sample_doc.iter() {
                                if key != "_id" {
                                    or_conditions.push(mongodb::bson::doc! {
                                        key: {"$regex": &state.filter_input, "$options": "i"}
                                    });
                                }
                            }

                            if or_conditions.is_empty() {
                                reload_documents_without_filter(state, conn_service).await;
                                return;
                            }

                            let filter = mongodb::bson::doc! {
                                "$or": or_conditions
                            };

                            match query_service
                                .find_documents(&db_name, &coll_name, Some(filter), 0, 20)
                                .await
                            {
                                Ok(documents) => {
                                    state.set_documents(documents);
                                    state.set_error(None);
                                }
                                Err(e) => {
                                    state.set_error(Some(format!("Search error: {}", e)));
                                }
                            }
                        } else {
                            state.set_documents(Vec::new());
                        }
                    }
                    Err(e) => {
                        state.set_error(Some(format!("Failed to analyze fields: {}", e)));
                    }
                }
            }
        }
    }

    // helper function to reload without filter
    async fn reload_documents_without_filter(
        state: &mut AppState,
        conn_service: &ConnectionService,
    ) {
        let db_name = state.current_database.clone();
        let coll_name = state.current_collection.clone();

        if let (Some(db_name), Some(coll_name)) = (db_name, coll_name) {
            if let Some(client) = conn_service.get_client() {
                let query_service = QueryService::new(client.clone());
                match query_service
                    .find_documents(&db_name, &coll_name, None, 0, 20)
                    .await
                {
                    Ok(documents) => {
                        state.set_documents(documents);
                        state.set_error(None);
                    }
                    Err(e) => {
                        state.set_error(Some(format!("Failed to reload: {}", e)));
                    }
                }
            }
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
