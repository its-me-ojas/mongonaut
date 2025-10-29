mod error;
mod models;
mod services;

use services::connection::ConnectionService;
use services::query::QueryService;

#[tokio::main]
async fn main() {
    println!("Mongonaut - MongoDB TUI Client");
    println!("Testing query service...\n");

    let mut conn_service = ConnectionService::new();
    let uri = "mongodb://localhost:27017";
    
    match conn_service.connect(uri).await {
        Ok(server_info) => {
            println!("✓ Connected to MongoDB {}\n", server_info.version);
            
            if let Some(client) = conn_service.get_client() {
                let query_service = QueryService::new(client.clone());
                
                println!("📚 Listing databases:");
                match query_service.list_databases().await {
                    Ok(databases) => {
                        for db in &databases {
                            println!("  - {} (size: {} bytes, empty: {})", 
                                db.name, db.size_on_disk, db.empty);
                        }
                        println!("  Total: {} databases\n", databases.len());
                        
                        if let Some(db) = databases.iter().find(|d| !d.name.starts_with("admin") && !d.name.starts_with("local") && !d.name.starts_with("config")) {
                            println!("📁 Listing collections in '{}':", db.name);
                            match query_service.list_collections(&db.name).await {
                                Ok(collections) => {
                                    for coll in &collections {
                                        println!("  - {} ({} documents, {} indexes)", 
                                            coll.name, coll.document_count, coll.indexes.len());
                                    }
                                    println!("  Total: {} collections\n", collections.len());
                                    
                                    if let Some(coll) = collections.first() {
                                        println!("📄 Querying documents from '{}.{}':", db.name, coll.name);
                                        match query_service.find_documents(&db.name, &coll.name, None, 0, 5).await {
                                            Ok(docs) => {
                                                println!("  Found {} documents (showing first 5)", docs.len());
                                                for (i, doc) in docs.iter().enumerate() {
                                                    println!("  Document {}: {} bytes", i + 1, doc.to_string().len());
                                                }
                                                
                                                match query_service.count_documents(&db.name, &coll.name, None).await {
                                                    Ok(count) => println!("\n  Total documents in collection: {}", count),
                                                    Err(e) => println!("\n  ✗ Count failed: {}", e),
                                                }
                                            }
                                            Err(e) => println!("  ✗ Query failed: {}", e),
                                        }
                                    }
                                }
                                Err(e) => println!("  ✗ Failed to list collections: {}", e),
                            }
                        } else {
                            println!("ℹ️  No user databases found. Create a database with some data to test queries.");
                        }
                    }
                    Err(e) => println!("✗ Failed to list databases: {}", e),
                }
            }
        }
        Err(e) => {
            println!("✗ Connection failed: {}", e);
        }
    }
}
