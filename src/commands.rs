use inquire::{Confirm, Text};

use crate::config::LocalConfig;

pub struct InitialCommand {}

impl InitialCommand {
    pub fn init(
        name: Option<String>,
        connection_type: Option<String>,
        steamcmd_path: Option<String>,
        workshop_path: Option<String>,
        server_path: Option<String>,
        hostname: Option<String>,
        username: Option<String>,
        password: Option<String>,
        port: Option<u16>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = LocalConfig::load_config()?;

        if config.is_initialized {
            println!("Configure your server");
            if let Some(name) = name {
                print!("Name: {}", name);
            } else {
                Text::new("Enter a name for your server")
                    .with_help_message("The name of your Server e.g. DayZFrostlineServer")
                    .prompt()?;
            }
        } else {
            println!("Initialize config");
            if let Some(name) = name {
                print!("Name: {}", name);
            } else {
                Text::new("Enter a name for your server")
                    .with_help_message("The name of your Server e.g. DayZFrostlineServer")
                    .prompt()?;
            }
        }

        Ok(())
    }
}
