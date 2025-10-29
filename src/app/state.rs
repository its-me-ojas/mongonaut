use std::fmt::format;

use super::screen::Screen;
use crate::models::{CollectionInfo, DatabaseInfo, ServerInfo};
use mongodb::bson::Document;

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub uri: String,
    pub server_info: ServerInfo,
}

#[derive(Debug)]
pub struct AppState {
    pub connection: Option<ConnectionState>,
    pub current_database: Option<String>,
    pub current_collection: Option<String>,
    pub databases: Vec<DatabaseInfo>,
    pub collections: Vec<CollectionInfo>,
    pub documents: Vec<Document>,
    pub current_page: usize,
    pub page_size: usize,
    pub filter: Option<Document>,
    pub loading: bool,
    pub error: Option<String>,
    pub should_quit: bool,
    pub selected_db_index: usize,
    pub selected_coll_index: usize,
    pub current_screen: Screen,
    pub selected_doc_index: usize,
    pub doc_scroll_offset: usize,
    pub connection_input: String,
    pub input_mode: bool,
    pub filter_input: String,
    pub filter_mode: bool,
    pub query_mode: bool,
    pub query_input: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            connection: None,
            current_database: None,
            current_collection: None,
            databases: Vec::new(),
            collections: Vec::new(),
            documents: Vec::new(),
            current_page: 0,
            page_size: 20,
            filter: None,
            loading: false,
            error: None,
            should_quit: false,
            selected_db_index: 0,
            selected_coll_index: 0,
            current_screen: Screen::Connection,
            selected_doc_index: 0,
            doc_scroll_offset: 0,
            connection_input: String::from("mongodb://localhost:27017"),
            input_mode: false,
            filter_input: String::new(),
            filter_mode: false,
            query_input: String::new(),
            query_mode: false,
        }
    }

    pub fn set_connection(&mut self, uri: String, server_info: ServerInfo) {
        self.connection = Some(ConnectionState { uri, server_info });
    }

    pub fn set_databases(&mut self, databases: Vec<DatabaseInfo>) {
        self.databases = databases;
        self.selected_db_index = 0;
    }

    pub fn set_collections(&mut self, collections: Vec<CollectionInfo>) {
        self.collections = collections;
        self.selected_coll_index = 0;
    }

    pub fn set_documents(&mut self, documents: Vec<Document>) {
        self.documents = documents;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    pub fn select_next_db(&mut self) {
        if !self.databases.is_empty() {
            self.selected_db_index = (self.selected_db_index + 1) % self.databases.len();
        }
    }

    pub fn select_prev_db(&mut self) {
        if !self.databases.is_empty() {
            if self.selected_db_index == 0 {
                self.selected_db_index = self.databases.len() - 1;
            } else {
                self.selected_db_index -= 1;
            }
        }
    }

    pub fn select_next_coll(&mut self) {
        if !self.collections.is_empty() {
            self.selected_coll_index = (self.selected_coll_index + 1) % self.collections.len();
        }
    }

    pub fn select_prev_coll(&mut self) {
        if !self.collections.is_empty() {
            if self.selected_coll_index == 0 {
                self.selected_coll_index = self.collections.len() - 1;
            } else {
                self.selected_coll_index -= 1;
            }
        }
    }

    pub fn get_selected_database(&self) -> Option<&DatabaseInfo> {
        self.databases.get(self.selected_db_index)
    }

    pub fn get_selected_collection(&self) -> Option<&CollectionInfo> {
        self.collections.get(self.selected_coll_index)
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.current_screen = screen
    }

    pub fn select_next_doc(&mut self) {
        if !self.documents.is_empty() {
            self.selected_doc_index = (self.selected_doc_index + 1) % self.documents.len();
        }
    }

    pub fn select_prev_doc(&mut self) {
        if !self.documents.is_empty() {
            if self.selected_doc_index == 0 {
                self.selected_doc_index == self.documents.len() - 1;
            } else {
                self.selected_doc_index -= 1;
            }
        }
    }

    pub fn scroll_doc_down(&mut self) {
        self.doc_scroll_offset += 1;
    }

    pub fn scroll_doc_up(&mut self) {
        if self.doc_scroll_offset > 0 {
            self.doc_scroll_offset -= 1;
        }
    }

    pub fn get_selected_document(&self) -> Option<&mongodb::bson::Document> {
        self.documents.get(self.selected_doc_index)
    }

    pub fn enter_input_mode(&mut self) {
        self.input_mode = true;
    }

    pub fn exit_input_mode(&mut self) {
        self.input_mode = false;
    }

    pub fn clear_input(&mut self) {
        self.connection_input.clear();
    }

    pub fn push_char(&mut self, c: char) {
        self.connection_input.push(c);
    }

    pub fn pop_char(&mut self) {
        self.connection_input.pop();
    }

    pub fn enter_filter_mode(&mut self) {
        self.filter_mode = true;
    }

    pub fn exit_filter_mode(&mut self) {
        self.filter_mode = false;
    }

    pub fn clear_filter(&mut self) {
        self.filter_input.clear();
        self.filter = None;
    }

    pub fn push_filter_char(&mut self, c: char) {
        self.filter_input.push(c);
    }

    pub fn pop_filter_char(&mut self) {
        self.filter_input.pop();
    }

    pub fn apply_filter(&mut self) -> Result<(), String> {
        let input = if !self.query_input.is_empty() {
            &self.query_input
        } else {
            &self.filter_input
        };

        if input.is_empty() {
            self.filter = None;
            return Ok(());
        }

        match serde_json::from_str::<serde_json::Value>(input) {
            Ok(json_value) => match mongodb::bson::to_document(&json_value) {
                Ok(doc) => {
                    self.filter = Some(doc);
                    Ok(())
                }
                Err(e) => Err(format!("Invalid filter: {}", e)),
            },
            Err(e) => Err(format!("Invalid JSON: {}", e)),
        }
    }

    pub fn enter_query_mode(&mut self) {
        self.query_mode = true;
    }

    pub fn exit_query_mode(&mut self) {
        self.query_mode = false;
    }

    pub fn push_every_char(&mut self, c: char) {
        self.query_input.push(c);
    }

    pub fn pop_every_char(&mut self) {
        self.query_input.pop();
    }

    pub fn clear_query(&mut self) {
        self.query_input.clear();
    }

    pub fn push_query_char(&mut self, c: char) {
        self.query_input.push(c);
    }

    pub fn pop_query_char(&mut self) {
        self.query_input.pop();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
