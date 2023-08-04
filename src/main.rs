// main.rs
mod commands;
use clap::{Args, Parser, Subcommand};
use commands::{ls, add, rm, push};

// Shared bin controller
#[derive(Parser)]
#[command(version = "1.0", author = "Rishi Shah <shahrishi108@gmail.com>", about = "HTTP bin CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists files in shbin
    Ls,

    /// Adds file to shbin
    Add (PathArgs),

    /// Removes file from shbin
    Rm (PathArgs),

    /// Pushes files in shbin to server
    Push,  
}

// TODO: add args to documentation
#[derive(Args)]
struct PathArgs {
    /// path to file
    #[arg(name = "path to file")]
    path: String,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Ls => {
            ls();
        },
        Commands::Add(path) => {
            add(&path.path);
        },
        Commands::Rm(path) => {
            rm(&path.path);
        },
        Commands::Push => {
            push();
        },
    }
}

