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
            current_screen: Screen::DatabaseList,
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
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
