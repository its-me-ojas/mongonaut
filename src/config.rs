use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionHistory {
    pub uris: Vec<String>,
}

impl ConnectionHistory {
    pub fn new() -> Self {
        Self { uris: Vec::new() }
    }

    pub fn add_uri(&mut self, uri: String) {
        if !self.uris.contains(&uri) {
            self.uris.insert(0, uri);
            if self.uris.len() > 10 {
                self.uris.truncate(10);
            }
        } else {
            self.uris.retain(|u| u != &uri);
            self.uris.insert(0, uri);
        }
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(history) = serde_json::from_str(&contents) {
                    return history;
                }
            }
        }
        Self::new()
    }

    fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home| home.join(".mongonaut").join("history.json"))
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let json = serde_json::to_string_pretty(self)?;
            fs::write(path, json)?;
        }
        Ok(())
    }
}
