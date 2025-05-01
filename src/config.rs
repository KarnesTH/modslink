use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub mods: Vec<Mods>,
    pub mode: Mode,
    pub mods_path: Option<String>,
    pub work_dir: Option<String>,
    pub steamcmd_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Mods {
    pub name: String,
    pub mod_id: String,
}

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Local,
    Steam,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Local
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mods: Vec::new(),
            mode: Mode::default(),
            mods_path: None,
            work_dir: None,
            steamcmd_path: None,
        }
    }
}

impl Default for Mods {
    fn default() -> Self {
        Mods {
            name: String::new(),
            mod_id: String::new(),
        }
    }
}

impl Config {
    pub fn load_config(&self) -> Result<Self, std::io::Error> {
        todo!("Implement load config functionality")
    }

    fn get_config_path(&self) -> Result<PathBuf, std::io::Error> {
        let current_dir = std::env::current_dir()?;
        let mut config_path = current_dir;
        config_path.push("modslink.toml");
        if config_path.exists() {
            Ok(config_path)
        } else {
            std::fs::create_dir_all(&config_path)?;
            Ok(config_path)
        }
    }
}
