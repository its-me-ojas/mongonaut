use mongodb::bson::Document;
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

#[derive(Debug,Clone)]
pub struct QueryParams{
    pub database: String,
    pub collection:String,
    pub filter: Option<Document>,
    pub skip: u64,
    pub limit: i64,
    pub sort: Option<Document>,
}

#[derive(Debug,Clone)]
pub struct QueryResult{
    pub document: Vec<Document>,
    pub total_count:u64,
    pub execution_time:Duration,
}
