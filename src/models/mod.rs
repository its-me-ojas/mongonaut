use crossterm::event::KeyCode;
use mongodb::bson::Document;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub name: String,
    pub size_on_disk: u64,
    pub collection_count: usize,
    pub empty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub name: String,
    pub document_count: u64,
    pub size: u64,
    pub indexes: Vec<String>,
    pub capped: bool,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub version: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct IndexInfo {
    pub name: String,
    pub keys: Document,
    pub unique: bool,
}

#[derive(Debug, Clone)]
pub struct QueryParams {
    pub database: String,
    pub collection: String,
    pub filter: Option<Document>,
    pub skip: u64,
    pub limit: i64,
    pub sort: Option<Document>,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub document: Vec<Document>,
    pub total_count: u64,
    pub execution_time: Duration,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub default_page_size: usize,
    pub cache_ttl_seconds: u64,
    pub connection_timeout_seconds: u64,
    pub theme: Theme,
    pub keybindings: KeyBindings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_page_size: 20,
            cache_ttl_seconds: 300,
            connection_timeout_seconds: 5,
            theme: Theme::default(),
            keybindings: KeyBindings::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub highlight_color: Color,
    pub error_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary_color: Color::Cyan,
            secondary_color: Color::Gray,
            highlight_color: Color::Yellow,
            error_color: Color::Red,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyBindings {
    pub quit: Vec<KeyCode>,
    pub navigate_up: Vec<KeyCode>,
    pub navigate_down: Vec<KeyCode>,
    pub select: Vec<KeyCode>,
    pub back: Vec<KeyCode>,
    pub refresh: Vec<KeyCode>,
    pub filter: Vec<KeyCode>,
    pub export: Vec<KeyCode>,
    pub help: Vec<KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: vec![KeyCode::Char('q'), KeyCode::Esc],
            navigate_up: vec![KeyCode::Up, KeyCode::Char('k')],
            navigate_down: vec![KeyCode::Down, KeyCode::Char('j')],
            select: vec![KeyCode::Enter],
            back: vec![KeyCode::Backspace, KeyCode::Char('h')],
            refresh: vec![KeyCode::Char('r')],
            filter: vec![KeyCode::Char('/')],
            export: vec![KeyCode::Char('e')],
            help: vec![KeyCode::Char('?')],
        }
    }
}
