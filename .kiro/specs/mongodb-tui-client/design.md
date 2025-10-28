# Design Document

## Overview

The MongoDB TUI Client (name: `mongonaut`) is a terminal-based application built with Rust and the ratatui library. The application follows a layered architecture separating UI rendering, business logic, and data access. It uses the official MongoDB Rust driver for database operations and implements an event-driven architecture for responsive keyboard interaction.

### Technology Stack

- **Language**: Rust (stable channel, edition 2021)
- **TUI Framework**: ratatui (formerly tui-rs) for terminal rendering
- **Backend**: crossterm for cross-platform terminal manipulation
- **MongoDB Driver**: mongodb crate (official Rust driver)
- **Async Runtime**: tokio for asynchronous operations
- **Serialization**: serde and serde_json for document handling
- **Configuration**: config crate for settings management

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        TUI Layer                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Screens    │  │   Widgets    │  │   Layouts    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  App State   │  │Event Handler │  │  Navigation  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Service Layer                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Connection  │  │Query Service │  │Export Service│      │
│  │   Service    │  │              │  │              │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      Data Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │MongoDB Driver│  │  Repository  │  │    Cache     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Component Interaction Flow

```
User Input → Event Handler → App State Update → Service Layer → MongoDB
                                    ↓
                              UI Re-render ← State Change
```

## Components and Interfaces

### 1. TUI Layer

#### Screen Manager

Manages different application screens and transitions between them.

```rust
pub enum Screen {
    Connection,
    DatabaseList,
    CollectionList,
    DocumentView,
    QueryEditor,
    AggregationEditor,
    Help,
}

pub struct ScreenManager {
    current_screen: Screen,
    screen_stack: Vec<Screen>,
}

impl ScreenManager {
    pub fn push_screen(&mut self, screen: Screen);
    pub fn pop_screen(&mut self) -> Option<Screen>;
    pub fn current(&self) -> &Screen;
}
```

#### Widget Components

**DatabaseListWidget**: Displays databases with metadata

- Renders scrollable list of databases
- Shows collection count and size
- Highlights selected database

**CollectionListWidget**: Displays collections within a database

- Renders collection names with document counts
- Shows indexes and size information
- Supports filtering by name

**DocumentViewerWidget**: Renders documents in formatted JSON

- Syntax highlighting for JSON
- Pagination controls
- Search and filter bar

**StatusBarWidget**: Shows connection status and keyboard hints

- Connection indicator
- Current operation status
- Context-sensitive keyboard shortcuts

### 2. Application Layer

#### App State

Central state management for the application.

```rust
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
}

pub struct ConnectionState {
    pub uri: String,
    pub client: mongodb::Client,
    pub server_info: ServerInfo,
}
```

#### Event Handler

Processes keyboard input and dispatches actions.

```rust
pub enum AppEvent {
    KeyPress(KeyEvent),
    Tick,
    Quit,
}

pub enum Action {
    Navigate(Direction),
    SelectItem,
    GoBack,
    Refresh,
    OpenFilter,
    OpenQuery,
    Export,
    Delete,
    Edit,
    ShowHelp,
}

pub struct EventHandler {
    pub fn handle_event(&self, event: AppEvent, state: &mut AppState) -> Result<Vec<Action>>;
    pub fn handle_action(&self, action: Action, state: &mut AppState) -> Result<()>;
}
```

#### Navigation Manager

Manages navigation state and history.

```rust
pub struct NavigationManager {
    history: Vec<NavigationState>,
    current_index: usize,
}

pub struct NavigationState {
    screen: Screen,
    database: Option<String>,
    collection: Option<String>,
    scroll_position: usize,
}
```

### 3. Service Layer

#### Connection Service

Manages MongoDB connections and authentication.

```rust
pub struct ConnectionService {
    client: Option<mongodb::Client>,
}

impl ConnectionService {
    pub async fn connect(&mut self, uri: &str) -> Result<ServerInfo>;
    pub async fn disconnect(&mut self) -> Result<()>;
    pub fn is_connected(&self) -> bool;
    pub async fn test_connection(&self) -> Result<bool>;
}
```

#### Query Service

Executes queries and aggregations.

```rust
pub struct QueryService {
    client: mongodb::Client,
}

impl QueryService {
    pub async fn list_databases(&self) -> Result<Vec<DatabaseInfo>>;
    pub async fn list_collections(&self, db: &str) -> Result<Vec<CollectionInfo>>;
    pub async fn find_documents(
        &self,
        db: &str,
        collection: &str,
        filter: Option<Document>,
        skip: u64,
        limit: i64,
    ) -> Result<Vec<Document>>;
    pub async fn aggregate(
        &self,
        db: &str,
        collection: &str,
        pipeline: Vec<Document>,
    ) -> Result<Vec<Document>>;
    pub async fn count_documents(
        &self,
        db: &str,
        collection: &str,
        filter: Option<Document>,
    ) -> Result<u64>;
}
```

#### CRUD Service

Handles document create, update, and delete operations.

```rust
pub struct CrudService {
    client: mongodb::Client,
}

impl CrudService {
    pub async fn insert_document(
        &self,
        db: &str,
        collection: &str,
        document: Document,
    ) -> Result<ObjectId>;
    pub async fn update_document(
        &self,
        db: &str,
        collection: &str,
        filter: Document,
        update: Document,
    ) -> Result<u64>;
    pub async fn delete_document(
        &self,
        db: &str,
        collection: &str,
        filter: Document,
    ) -> Result<u64>;
}
```

#### Export Service

Handles data export to various formats.

```rust
pub enum ExportFormat {
    Json,
    Csv,
    Bson,
}

pub struct ExportService;

impl ExportService {
    pub async fn export_documents(
        &self,
        documents: &[Document],
        path: &Path,
        format: ExportFormat,
    ) -> Result<usize>;
}
```

#### Collection Service

Manages collection-level operations.

```rust
pub struct CollectionService {
    client: mongodb::Client,
}

impl CollectionService {
    pub async fn create_collection(&self, db: &str, name: &str) -> Result<()>;
    pub async fn drop_collection(&self, db: &str, name: &str) -> Result<()>;
    pub async fn rename_collection(&self, db: &str, old_name: &str, new_name: &str) -> Result<()>;
    pub async fn list_indexes(&self, db: &str, collection: &str) -> Result<Vec<IndexInfo>>;
    pub async fn create_index(&self, db: &str, collection: &str, keys: Document) -> Result<String>;
}
```

### 4. Data Layer

#### Repository Pattern

Abstracts MongoDB operations for testability.

```rust
#[async_trait]
pub trait MongoRepository {
    async fn get_databases(&self) -> Result<Vec<DatabaseInfo>>;
    async fn get_collections(&self, db: &str) -> Result<Vec<CollectionInfo>>;
    async fn query_documents(&self, query: QueryParams) -> Result<QueryResult>;
}

pub struct MongoRepositoryImpl {
    client: mongodb::Client,
}
```

#### Cache Layer

Caches frequently accessed data to reduce MongoDB queries.

```rust
pub struct CacheService {
    database_cache: HashMap<String, (Vec<DatabaseInfo>, Instant)>,
    collection_cache: HashMap<String, (Vec<CollectionInfo>, Instant)>,
    ttl: Duration,
}

impl CacheService {
    pub fn get_databases(&self) -> Option<&Vec<DatabaseInfo>>;
    pub fn set_databases(&mut self, databases: Vec<DatabaseInfo>);
    pub fn invalidate(&mut self);
}
```

## Data Models

### Core Data Structures

```rust
pub struct DatabaseInfo {
    pub name: String,
    pub size_on_disk: u64,
    pub collection_count: usize,
    pub empty: bool,
}

pub struct CollectionInfo {
    pub name: String,
    pub document_count: u64,
    pub size: u64,
    pub indexes: Vec<String>,
    pub capped: bool,
}

pub struct ServerInfo {
    pub version: String,
    pub host: String,
    pub port: u16,
}

pub struct IndexInfo {
    pub name: String,
    pub keys: Document,
    pub unique: bool,
}

pub struct QueryParams {
    pub database: String,
    pub collection: String,
    pub filter: Option<Document>,
    pub skip: u64,
    pub limit: i64,
    pub sort: Option<Document>,
}

pub struct QueryResult {
    pub documents: Vec<Document>,
    pub total_count: u64,
    pub execution_time: Duration,
}
```

### Configuration Model

```rust
pub struct AppConfig {
    pub default_page_size: usize,
    pub cache_ttl_seconds: u64,
    pub connection_timeout_seconds: u64,
    pub theme: Theme,
    pub keybindings: KeyBindings,
}

pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub highlight_color: Color,
    pub error_color: Color,
}

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
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("MongoDB error: {0}")]
    Mongo(#[from] mongodb::error::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### Error Display Strategy

- Connection errors: Display in status bar with retry option
- Query errors: Show in modal dialog with error details
- IO errors: Display with file path and permission information
- Validation errors: Inline display near input fields
- Fatal errors: Full-screen error with stack trace option

## Testing Strategy

### Unit Testing

**Service Layer Tests**

- Mock MongoDB client using mockall crate
- Test query construction and result parsing
- Test error handling and edge cases
- Test cache invalidation logic

**Event Handler Tests**

- Test keyboard input mapping to actions
- Test state transitions
- Test navigation history

**Export Service Tests**

- Test format conversion (JSON, CSV, BSON)
- Test file writing with various document structures
- Test large dataset handling

### Integration Testing

**MongoDB Integration Tests**

- Use testcontainers-rs for MongoDB instance
- Test actual database operations
- Test connection handling and reconnection
- Test query performance with realistic data volumes

**TUI Integration Tests**

- Use ratatui's TestBackend for UI testing
- Test screen rendering
- Test widget layout and responsiveness
- Test keyboard navigation flows

### Performance Testing

**Benchmarks**

- Document rendering performance (1K, 10K, 100K documents)
- Query execution time
- UI responsiveness under load
- Memory usage profiling

**Load Testing**

- Test with large collections (1M+ documents)
- Test with deeply nested documents
- Test with high-latency connections
- Test memory usage over extended sessions

### Manual Testing Checklist

- Connection to various MongoDB versions (4.x, 5.x, 6.x, 7.x)
- SSH tunnel connections
- MongoDB Atlas connections
- Keyboard navigation in all screens
- Export functionality with various formats
- CRUD operations on different document structures
- Aggregation pipeline execution
- Error recovery scenarios

## Performance Considerations

### Optimization Strategies

1. **Lazy Loading**: Load documents on-demand with pagination
2. **Virtual Scrolling**: Render only visible documents in viewport
3. **Connection Pooling**: Reuse MongoDB connections
4. **Caching**: Cache database and collection lists with TTL
5. **Async Operations**: Non-blocking UI during database operations
6. **Batch Operations**: Group multiple operations when possible

### Memory Management

- Limit in-memory document cache to 1000 documents
- Stream large query results instead of loading all at once
- Release resources when switching contexts
- Implement document size limits for display (truncate large documents)

### Rendering Optimization

- Use ratatui's StatefulWidget for efficient re-rendering
- Minimize full-screen redraws
- Update only changed regions
- Debounce rapid keyboard input

## Deployment and Distribution

### Build Configuration

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### Distribution Channels

1. **Cargo**: Publish to crates.io
2. **Binary Releases**: GitHub releases with pre-built binaries for:
   - Linux (x86_64, aarch64)
   - macOS (Intel, Apple Silicon)
   - Windows (x86_64)
3. **Package Managers**:
   - Homebrew (macOS/Linux)
   - Scoop (Windows)
   - AUR (Arch Linux)

### Installation Methods

```bash
# Via cargo
cargo install mongonaut

# Via homebrew
brew install mongonaut

# Via binary download
curl -L https://github.com/user/mongonaut/releases/latest/download/mongonaut-linux-x86_64 -o mongonaut
chmod +x mongonaut
```

## Future Enhancements (Post-MVP)

1. **Saved Queries**: Store and recall frequently used queries
2. **SSH Tunneling**: Built-in SSH tunnel support
3. **MongoDB Atlas**: Native Atlas API integration
4. **Multi-Database Support**: PostgreSQL, CouchDB adapters
5. **Query History**: Searchable query history
6. **Themes**: Customizable color schemes
7. **Plugins**: Extension system for custom functionality
8. **Collaboration**: Share queries and connections with team
9. **Performance Monitoring**: Real-time query performance metrics
10. **Data Visualization**: Charts and graphs for aggregation results
