use inquire::{Confirm, Select, Text};

use crate::config::{LocalConfig, ServerConnectionType};

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
            let name = Self::get_name(name)?;
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

    fn init_config() -> Result<(), Box<dyn std::error::Error>> {
        todo!("Implement initialize functionality");
    }

    fn get_name(name: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(name) = name {
            Ok(name)
        } else {
            Ok(Text::new("Enter a name for your server")
                .with_help_message("The name of your Server e.g. DayZFrostlineServer")
                .prompt()?)
        }
    }

    fn get_connection_type(
        connection_type: Option<String>,
    ) -> Result<ServerConnectionType, Box<dyn std::error::Error>> {
        if let Some(connection_type) = connection_type {
            match connection_type.as_str() {
                "local" => Ok(ServerConnectionType::Local),
                "ftp" => Ok(ServerConnectionType::Ftp),
                "sftp" => Ok(ServerConnectionType::Sftp),
                _ => Err("Invalid connection type".into()),
            }
        } else {
            let connection_types = vec!["local", "ftp", "sftp"];
            let selected_type = Select::new("Select a connection type", connection_types)
                .with_help_message("The type of connection to use")
                .prompt()?;
            match selected_type {
                "local" => Ok(ServerConnectionType::Local),
                "ftp" => Ok(ServerConnectionType::Ftp),
                "sftp" => Ok(ServerConnectionType::Sftp),
                _ => Err("Invalid connection type".into()),
            }
        }
    }

    fn get_steamcmd_path(
        steamcmd_path: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(path) = steamcmd_path {
            Ok(path)
        } else {
            Ok(Text::new("Enter the path to your SteamCMD installation")
                .with_help_message("The path to your SteamCMD installation")
                .prompt()?)
        }
    }

    fn get_workshop_path(
        workshop_path: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(path) = workshop_path {
            Ok(path)
        } else {
            Ok(Text::new("Enter the path to your workshop directory")
                .with_help_message("The path to your workshop directory")
                .prompt()?)
        }
    }

    fn get_server_path(server_path: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(path) = server_path {
            Ok(path)
        } else {
            Ok(Text::new("Enter the path to your server directory")
                .with_help_message("The path to your server directory")
                .prompt()?)
        }
    }

    fn get_hostname(hostname: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(hostname) = hostname {
            Ok(hostname)
        } else {
            Ok(Text::new("Enter the hostname of your FTP server")
                .with_help_message("The hostname of your FTP server")
                .prompt()?)
        }
    }

    fn get_username(username: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(username) = username {
            Ok(username)
        } else {
            Ok(Text::new("Enter the username for your FTP server")
                .with_help_message("The username for your FTP server")
                .prompt()?)
        }
    }

    fn get_password(password: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(password) = password {
            Ok(password)
        } else {
            Ok(Text::new("Enter the password for your FTP server")
                .with_help_message("The password for your FTP server")
                .prompt()?)
        }
    }

    fn get_port(port: Option<String>) -> Result<u16, Box<dyn std::error::Error>> {
        if let Some(port) = port {
            port.parse().map_err(|_| "Invalid port number".into())
        } else {
            Ok(Text::new("Enter the port number for your FTP server")
                .with_help_message("The port number for your FTP server")
                .prompt()?
                .parse()
                .map_err(|_| "Invalid port number".to_string())?)
        }
    }
}
