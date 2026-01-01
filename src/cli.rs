use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bforge")]
#[command(about = "Decentralized code injection tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize beforge config in the current directory
    Init,

    /// Add a new item to your project
    Add {
        /// The repository to add (e.g., username/repo)
        #[arg(index = 1)]
        repo: String,

        /// The specific item to add from the repository
        #[arg(index = 2)]
        item: String,

        /// Overwrite existing files if they exist
        #[arg(short, long)]
        force: bool,
    },
}
