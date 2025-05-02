use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocalConfig {
    pub steamcmd_path: String,
    pub servers: Vec<ServersConfig>,
    pub workshop_path: String,
    pub is_initialized: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ServersConfig {
    pub name: String,
    pub connection_type: ServerConnectionType,
    pub server_path: String,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ServerConnectionType {
    Local,
    Ftp,
    Sftp,
}

#[derive(Serialize, Deserialize)]
pub struct ModsLinkConfig {
    pub server_name: String,
    pub last_updated: String,
    pub mods: Vec<ModsConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ModsConfig {
    pub name: String,
    pub folder_name: String,
    pub mod_id: String,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            steamcmd_path: String::new(),
            servers: Vec::new(),
            workshop_path: String::new(),
            is_initialized: false,
        }
    }
}

impl LocalConfig {
    pub fn load_config() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        if config_path.exists() {
            let config = std::fs::read_to_string(config_path)?;
            toml::from_str(&config).map_err(|e| e.into())
        } else {
            Ok(Self::default())
        }
    }

    fn get_config_path() -> Result<PathBuf, std::io::Error> {
        let config_dir = dirs::config_dir().expect("Failed to get config directory");
        let config_path = config_dir
            .join("karnes-development")
            .join("modslink")
            .join("config.toml");
        Ok(config_path)
    }

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config = toml::to_string(&self)?;
        std::fs::write(config_path, config)?;
        Ok(())
    }
}
