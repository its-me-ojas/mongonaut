# Implementation Plan

- [x] 1. Initialize Rust project and configure dependencies

  - Create new Rust project with `cargo new mongonaut`
  - Add dependencies to Cargo.toml: ratatui, crossterm, tokio, mongodb, serde, serde_json, thiserror, config
  - Configure release profile for optimized builds (LTO, strip symbols)
  - Set up project structure with modules: ui, app, services, models, error
  - _Requirements: 8.1, 8.2_

- [x] 2. Implement core error handling and result types

  - Create AppError enum with variants for Connection, Query, IO, Mongo, Serialization, and InvalidInput errors
  - Implement Display and Error traits for AppError
  - Create type alias Result<T> for std::result::Result<T, AppError>
  - Add error conversion implementations using thiserror
  - _Requirements: 1.3_

- [x] 3. Implement data models and structures

  - [x] 3.1 Create core data model structs

    - Define DatabaseInfo struct with name, size_on_disk, collection_count, empty fields
    - Define CollectionInfo struct with name, document_count, size, indexes, capped fields
    - Define ServerInfo struct with version, host, port fields
    - Define IndexInfo struct with name, keys, unique fields
    - Define QueryParams and QueryResult structs
    - _Requirements: 2.3, 2.4, 10.1, 10.2_

  - [x] 3.2 Create configuration models
    - Define AppConfig struct with default_page_size, cache_ttl_seconds, connection_timeout_seconds, theme, keybindings
    - Define Theme struct with color fields
    - Define KeyBindings struct with keyboard shortcut mappings
    - Implement Default trait for configuration structs
    - _Requirements: 7.2, 7.3_

- [x] 4. Implement connection service

  - [x] 4.1 Create ConnectionService struct and methods

    - Implement connect method that accepts MongoDB URI and establishes connection
    - Implement disconnect method to close connection gracefully
    - Implement is_connected method to check connection status
    - Implement test_connection method to verify active connection
    - Add connection timeout handling (5 seconds)
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

  - [x] 4.2 Add connection error handling
    - Parse MongoDB connection errors and create user-friendly error messages
    - Handle authentication failures with specific error messages
    - Handle network timeout errors
    - Handle invalid URI format errors
    - _Requirements: 1.3_

- [x] 5. Implement query service for database operations

  - [x] 5.1 Create QueryService struct with MongoDB client

    - Implement list_databases method to fetch all databases
    - Implement list_collections method to fetch collections for a database
    - Implement find_documents method with filter, skip, and limit parameters
    - Implement count_documents method for pagination
    - Add query timeout handling (5 seconds for queries, 2 seconds for filters)
    - _Requirements: 2.1, 2.2, 3.1, 3.4_

  - [x] 5.2 Implement aggregation pipeline support
    - Create aggregate method accepting pipeline as Vec<Document>
    - Parse and validate aggregation pipeline syntax
    - Execute aggregation with timeout (5 seconds)
    - Return aggregation results as Vec<Document>
    - Handle aggregation errors with line number information
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 6. Implement CRUD service for document operations

  - Create CrudService struct with MongoDB client
  - Implement insert_document method returning ObjectId
  - Implement update_document method with filter and update parameters
  - Implement delete_document method with filter parameter
  - Add confirmation prompts for destructive operations
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 7. Implement collection management service

  - Create CollectionService struct with MongoDB client
  - Implement create_collection method with name and optional validation rules
  - Implement drop_collection method with confirmation
  - Implement rename_collection method with name conflict validation
  - Implement list_indexes method to fetch index information
  - Implement create_index and drop_index methods
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5, 10.4, 10.5_

- [ ] 8. Implement export service

  - [ ] 8.1 Create ExportService and format enum

    - Define ExportFormat enum with Json, Csv, Bson variants
    - Create ExportService struct
    - Implement export_documents method accepting documents, path, and format
    - _Requirements: 6.1, 6.2_

  - [ ] 8.2 Implement format-specific export logic
    - Implement JSON export with pretty printing
    - Implement CSV export with header row and field flattening
    - Implement BSON export using mongodb::bson
    - Add progress tracking for large exports
    - Handle export completion within 10 seconds for <10K documents
    - _Requirements: 6.3, 6.4, 6.5_

- [ ] 9. Implement caching layer

  - Create CacheService struct with HashMap storage
  - Implement get_databases and set_databases methods
  - Implement get_collections and set_collections methods
  - Add TTL-based cache invalidation
  - Implement manual cache invalidation method
  - _Requirements: 8.4_

- [ ] 10. Implement application state management

  - [ ] 10.1 Create AppState struct

    - Define AppState with connection, current_database, current_collection, databases, collections, documents fields
    - Add pagination fields: current_page, page_size (default 20)
    - Add filter, loading, error fields
    - Define ConnectionState struct with uri, client, server_info
    - _Requirements: 3.1, 3.5_

  - [ ] 10.2 Implement state update methods
    - Create methods to update connection state
    - Create methods to update current database and collection
    - Create methods to update document list with pagination
    - Create methods to set loading and error states
    - _Requirements: 2.1, 2.2, 3.1_

- [ ] 11. Implement event handling system

  - [ ] 11.1 Create event and action enums

    - Define AppEvent enum with KeyPress, Tick, Quit variants
    - Define Action enum with Navigate, SelectItem, GoBack, Refresh, OpenFilter, OpenQuery, Export, Delete, Edit, ShowHelp
    - Define Direction enum for navigation
    - _Requirements: 7.1, 7.4_

  - [ ] 11.2 Create EventHandler struct
    - Implement handle_event method to convert events to actions
    - Implement handle_action method to execute actions and update state
    - Add keyboard input processing with <50ms latency
    - Map keyboard shortcuts to actions based on current screen
    - _Requirements: 7.1, 7.3, 7.4, 7.5_

- [ ] 12. Implement screen management

  - Define Screen enum with Connection, DatabaseList, CollectionList, DocumentView, QueryEditor, AggregationEditor, Help variants
  - Create ScreenManager struct with current_screen and screen_stack
  - Implement push_screen, pop_screen, and current methods
  - Add screen transition logic
  - _Requirements: 2.1, 2.2, 3.1_

- [ ] 13. Implement navigation manager

  - Create NavigationManager struct with history and current_index
  - Define NavigationState struct with screen, database, collection, scroll_position
  - Implement navigation history push and pop methods
  - Implement forward and backward navigation
  - Store scroll positions for each screen
  - _Requirements: 2.5, 3.5, 7.1_

- [ ] 14. Build TUI widgets

  - [ ] 14.1 Create DatabaseListWidget

    - Implement StatefulWidget for database list rendering
    - Display database names with collection count and size
    - Add selection highlighting
    - Implement scrolling with keyboard navigation
    - _Requirements: 2.1, 2.3_

  - [ ] 14.2 Create CollectionListWidget

    - Implement StatefulWidget for collection list rendering
    - Display collection names with document count and size
    - Show index information
    - Add selection highlighting and scrolling
    - _Requirements: 2.2, 2.4_

  - [ ] 14.3 Create DocumentViewerWidget

    - Implement document rendering in formatted JSON
    - Add syntax highlighting for JSON (keys, values, brackets)
    - Implement pagination controls (20 documents per page)
    - Add scrolling for long documents
    - Display document index and total count
    - _Requirements: 3.1, 3.2, 3.5_

  - [ ] 14.4 Create StatusBarWidget

    - Display connection status (connected/disconnected, server info)
    - Show current operation status and loading indicator
    - Display context-sensitive keyboard shortcuts
    - Show error messages when present
    - _Requirements: 1.5, 7.2_

  - [ ] 14.5 Create FilterInputWidget

    - Create input field for filter queries
    - Add JSON syntax validation
    - Display filter errors inline
    - _Requirements: 3.3, 3.4_

  - [ ] 14.6 Create HelpWidget
    - Display all keyboard shortcuts organized by category
    - Show shortcuts for current screen context
    - Add scrolling for long help content
    - _Requirements: 7.2_

- [ ] 15. Implement connection screen

  - Create connection screen layout with URI input field
  - Add connection button and status display
  - Implement URI validation
  - Show connection progress indicator
  - Display server information on successful connection
  - Handle connection errors and display error messages
  - Transition to database list screen on successful connection
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [ ] 16. Implement database list screen

  - Create database list screen layout with DatabaseListWidget
  - Load databases on screen entry using QueryService
  - Implement keyboard navigation (up/down arrows, vim keys)
  - Handle database selection to transition to collection list
  - Add refresh functionality
  - Display loading state while fetching databases
  - Show database statistics
  - _Requirements: 2.1, 2.3, 2.5, 7.1, 7.3_

- [ ] 17. Implement collection list screen

  - Create collection list screen layout with CollectionListWidget
  - Load collections for selected database
  - Implement keyboard navigation
  - Handle collection selection to transition to document view
  - Add collection management options (create, drop, rename)
  - Display collection statistics and indexes
  - Add refresh functionality
  - _Requirements: 2.2, 2.4, 2.5, 9.1, 9.2, 9.3, 9.4, 10.2, 10.4_

- [ ] 18. Implement document view screen

  - [ ] 18.1 Create document view layout

    - Create screen layout with DocumentViewerWidget and filter bar
    - Load initial documents (page 1, 20 documents)
    - Display pagination information (current page, total documents)
    - _Requirements: 3.1, 3.5_

  - [ ] 18.2 Add document navigation and filtering

    - Implement page navigation (next/previous page)
    - Add filter input and apply filter functionality
    - Implement document scrolling
    - Add document selection for CRUD operations
    - _Requirements: 3.3, 3.4, 3.5_

  - [ ] 18.3 Add document CRUD operations
    - Add edit document functionality (open JSON editor)
    - Add delete document functionality with confirmation
    - Add create new document functionality
    - Add duplicate document functionality
    - Update document list after operations
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 19. Implement aggregation editor screen

  - Create aggregation editor layout with multi-line input
  - Add JSON syntax validation for pipeline stages
  - Implement pipeline execution
  - Display aggregation results in DocumentViewerWidget
  - Show execution time
  - Handle aggregation errors with line numbers
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 20. Implement export functionality

  - Add export keyboard shortcut handler
  - Create export dialog for format selection (JSON, CSV, BSON)
  - Add file path input
  - Execute export using ExportService
  - Display export progress for large datasets
  - Show completion message with file path and document count
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [ ] 21. Implement help screen

  - Create help screen layout with HelpWidget
  - Organize shortcuts by category (Navigation, Actions, Screens)
  - Display context-sensitive help based on current screen
  - Add scrolling for help content
  - Implement help toggle (show/hide)
  - _Requirements: 7.2_

- [ ] 22. Implement main application loop

  - [ ] 22.1 Create main.rs entry point

    - Initialize terminal with crossterm backend
    - Create AppState and EventHandler
    - Set up tokio runtime for async ope
      rations
    - Initialize ratatui Terminal
    - _Requirements: 8.1, 8.3_

  - [ ] 22.2 Implement event loop

    - Create event polling loop with tick rate
    - Handle keyboard events and dispatch to EventHandler
    - Update AppState based on actions
    - Render UI updates with <100ms latency
    - Handle quit event and cleanup terminal
    - _Requirements: 7.5, 8.3_

  - [ ] 22.3 Add async task handling
    - Spawn async tasks for MongoDB operations
    - Update UI state when async operations complete
    - Handle async operation errors
    - Maintain UI responsiveness during operations
    - _Requirements: 8.3, 8.4_

- [ ] 23. Implement configuration management

  - Create default configuration file structure
  - Implement configuration loading from file (~/.mongonaut/config.toml)
  - Add configuration validation
  - Support custom keybindings from config
  - Support theme customization from config
  - _Requirements: 7.2, 7.3_

- [ ] 24. Add performance optimizations

  - Implement virtual scrolling for large document lists
  - Add lazy loading for documents (load on scroll)
  - Optimize JSON rendering for large documents (truncate if needed)
  - Implement connection pooling
  - Add document size limits for display
  - Profile memory usage and optimize
  - _Requirements: 8.1, 8.2, 8.3, 8.4_

- [ ] 25. Create build and release configuration

  - Configure Cargo.toml release profile with optimizations
  - Create build scripts for cross-platform compilation
  - Set up GitHub Actions for CI/CD
  - Create release binaries for Linux (x86_64, aarch64), macOS (Intel, Apple Silicon), Windows
  - Write installation documentation
  - _Requirements: 8.1, 8.2_

- [ ] 26. Write user documentation
  - Create README.md with project overview and features
  - Write installation instructions for different platforms
  - Document connection URI format and examples
  - Create keyboard shortcuts reference
  - Add usage examples and screenshots
  - Document configuration options
  - _Requirements: 1.1, 7.2_
