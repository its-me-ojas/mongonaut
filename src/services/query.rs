use mongodb::{Client, Database, bson::Document, results};
use tokio::fs::File;

use crate::{
    error::AppError,
    models::{CollectionInfo, DatabaseInfo},
};

pub struct QueryService {
    client: Client,
}

impl QueryService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list_databases(&self) -> Result<Vec<DatabaseInfo>, AppError> {
        let databases = self
            .client
            .list_databases()
            .await
            .map_err(|e| AppError::Connection(format!("Failed to list databases: {}", e)))?;

        let mut db_infos = Vec::new();
        for db in databases {
            let db_info = DatabaseInfo {
                name: db.name,
                size_on_disk: db.size_on_disk as u64,
                collection_count: 0,
                empty: db.empty,
            };
            db_infos.push(db_info);
        }

        Ok(db_infos)
    }

    pub async fn list_collections(&self, db: &str) -> Result<Vec<CollectionInfo>, AppError> {
        let database = self.client.database(db);
        let collections = database
            .list_collection_names()
            .await
            .map_err(|e| AppError::Query(format!("Failed to list collections: {}", e)))?;

        let mut coll_infos = Vec::new();
        for coll_name in collections {
            let collection = database.collection::<Document>(&coll_name);

            let doc_count = collection.estimated_document_count().await.unwrap_or(0);

            let indexes = collection.list_index_names().await.unwrap_or_default();

            let coll_info = CollectionInfo {
                name: coll_name,
                document_count: doc_count,
                size: 0,
                indexes,
                capped: false,
            };
            coll_infos.push(coll_info);
        }
        Ok(coll_infos)
    }

    pub async fn find_documents(
        &self,
        db: &str,
        collection: &str,
        filter: Option<Document>,
        skip: u64,
        limit: i64,
    ) -> Result<Vec<Document>, AppError> {
        let coll = self.client.database(db).collection::<Document>(collection);

        let filter_doc = filter.unwrap_or_else(|| Document::new());

        let mut cursor = coll
            .find(filter_doc)
            .skip(skip)
            .limit(limit)
            .await
            .map_err(|e| AppError::Query(format!("Failed to find documents: {}", e)))?;

        let mut documents = Vec::new();
        use futures::stream::StreamExt;
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => documents.push(doc),
                Err(e) => {
                    return Err(AppError::Query(format!(
                        "Error reading aggregation result: {}",
                        e
                    )));
                }
            }
        }
        Ok(documents)
    }
    
    pub async fn count_documents(
    &self,
    db: &str,
    collection: &str,
    filter: Option<Document>,
) -> Result<u64, AppError> {
    let coll = self.client.database(db).collection::<Document>(collection);

    let filter_doc = filter.unwrap_or_else(|| Document::new());

    let count = coll
        .count_documents(filter_doc)
        .await
        .map_err(|e| AppError::Query(format!("Failed to count documents: {}", e)))?;

    Ok(count)
}

    pub async fn aggregate(
    &self,
    db: &str,
    collection: &str,
    pipeline: Vec<Document>,
) -> Result<Vec<Document>, AppError> {
    let coll = self.client.database(db).collection::<Document>(collection);

    let mut cursor = coll
        .aggregate(pipeline)
        .await
        .map_err(|e| AppError::Query(format!("Aggregation failed: {}", e)))?;

    let mut documents = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => documents.push(doc),
            Err(e) => return Err(AppError::Query(format!("Error reading aggregation result: {}", e))),
        }
    }

    Ok(documents)
}

}
