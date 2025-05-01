use clap::Parser;

mod cli;
mod commands;
mod config;
use cli::{Cli, ModCommands};

fn main() {
    let args = Cli::parse();
    match args.commands {
        ModCommands::Add => {
            println!("Adding a new module...");
        }
        ModCommands::Remove => {
            println!("Removing a module...");
        }
        ModCommands::List => {
            println!("Listing all modules...");
        }
        ModCommands::Update => {
            println!("Updating a module...");
        }
        ModCommands::Init => {
            println!("Initializing a new project...");
        }
        ModCommands::Debug => {
            println!("Debugging...");
        }
    }
}
