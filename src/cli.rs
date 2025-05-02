use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(author = "KarnesTH", version = VERSION, about = "A CLI tool for managing mods")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: ModCommands,
}

#[derive(Parser)]
pub enum ModCommands {
    #[clap(name = "init", about = "Initialize a new modslink project")]
    Init {
        #[clap(long, help = "The path to your SteamCMD installation")]
        steamcmd_path: Option<String>,
        #[clap(long, help = "The path to your !Workshop folder")]
        workshop_path: Option<String>,
        #[clap(long, help = "The Server name of your Server")]
        name: Option<String>,
        #[clap(long, help = "Server connection Type")]
        connection_type: Option<String>,
        #[clap(
            long,
            help = "Server path (recommended if you want to use connection type local)"
        )]
        server_path: Option<String>,
        #[clap(long, help = "FTP hostname")]
        hostname: Option<String>,
        #[clap(long, help = "FTP username")]
        username: Option<String>,
        #[clap(long, help = "FTP password")]
        password: Option<String>,
        #[clap(long, help = "FTP port")]
        port: Option<u16>,
    },
    #[clap(name = "add", about = "Add a new mod to the project")]
    Add,
    #[clap(name = "remove", about = "Remove a mod from the project")]
    Remove,
    #[clap(name = "update", about = "Update a mod in the project")]
    Update,
    #[clap(name = "list", about = "List all mods in the project")]
    List,
    /// Help command to display debug informations for development purposes
    Debug,
}
