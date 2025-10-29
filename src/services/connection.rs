use std::time::Duration;

use mongodb::{Client, options::ClientOptions};

use crate::{error::AppError, models::ServerInfo};

pub struct ConnectionService {
    client: Option<Client>,
    server_info: Option<ServerInfo>,
}

impl ConnectionService {
    pub fn new() -> Self {
        Self {
            client: None,
            server_info: None,
        }
    }

    pub async fn connect(&mut self, uri: &str) -> Result<ServerInfo,AppError> {
        // parsing connection string
        let mut client_options = ClientOptions::parse(uri)
            .await
            .map_err(|e| AppError::Connection(format!("Invalid URI: {}", e)))?;

        // setting timeout
        client_options.connect_timeout = Some(Duration::from_secs(5));
        client_options.server_selection_timeout = Some(Duration::from_secs(5));

        let client = Client::with_options(client_options)
            .map_err(|e| AppError::Connection(format!("Failed to create client: {}", e)))?;

        client
            .database("admin")
            .run_command(mongodb::bson::doc! { "ping": 1 })
            .await
            .map_err(|e| AppError::Connection(format!("Connection failed: {}", e)))?;

        let build_info = client
            .database("admin")
            .run_command(mongodb::bson::doc! {"buildInfo":1})
            .await
            .map_err(|e| AppError::Connection(format!("Failed to get server info: {}", e)))?;
        let version = build_info
        .get_str("version").unwrap_or("unknown").to_string();

        let (host, port) = Self::parse_host_port(uri);
        let server_info = ServerInfo {
            version,
            host,
            port,
        };

        self.client = Some(client);
        self.server_info = Some(server_info.clone());

        Ok(server_info)
    }
    pub async fn disconnect(&mut self) -> Result<(),AppError> {
        self.client = None;
        self.server_info = None;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    pub async fn test_connection(&self) -> Result<bool,AppError> {
        if let Some(client) = &self.client {
            client
                .database("admin")
                .run_command(mongodb::bson::doc! { "ping": 1 })
                .await
                .map_err(|e| AppError::Connection(format!("Connection test failed: {}", e)))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn get_client(&self) -> Option<&Client> {
        self.client.as_ref()
    }
    pub fn get_server_info(&self) -> Option<ServerInfo> {
        self.server_info.clone()
    }
    fn parse_host_port(uri: &str) -> (String, u16) {
        if let Some(after_protocol) = uri
            .strip_prefix("mongodb://")
            .or_else(|| uri.strip_prefix("mongodb+srv://"))
        {
            if let Some(host_part) = after_protocol.split('/').next() {
                let host_part = if let Some(at_pos) = host_part.rfind('@') {
                    &host_part[at_pos + 1..]
                } else {
                    host_part
                };

                if let Some((host, port_str)) = host_part.split_once(':') {
                    let port = port_str.parse().unwrap_or(27017);
                    return (host.to_string(), port);
                } else {
                    return (host_part.to_string(), 27017);
                }
            }
        }
        ("localhost".to_string(), 27017)
    }
}
