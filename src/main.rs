extern crate core;

mod new;
mod ls;
mod file_helpers;
mod get;

use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::file_helpers::get_vault_path_string;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all stored secrets
    Ls,
    /// Create a new secret
    New {
        #[arg(short, long)]
        name: String
    },
    /// Get a secret and store in clipboard
    Get {
        #[arg(short, long)]
        name: String
    },
}

fn main() {
    let cli = Cli::parse();

    bootstrap();

    match &cli.commands {
        Some(Commands::Ls ) => {
            ls::ls();
        }
        Some(Commands::New {name }) => {
            new::new(name);
        }
        Some(Commands::Get {name }) => {
            get::get(name);
        }
        None => {}
    }
}

fn bootstrap () {
    let create_vault_path = get_vault_path_string();

    if !Path::exists(create_vault_path.as_ref()) {
        println!("No vault found. Setting up at {}", create_vault_path);
        fs::create_dir(create_vault_path).unwrap();
    }
}
