use bforge::{
    cli::{Cli, Commands},
    commands,
};
use clap::Parser;
use colored::Colorize;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            commands::handle_init();
        }
        bforge::cli::Commands::Add {
            repo,
            item,
            force: _force,
        } => {
            let res = commands::handle_add(repo, item);
            if let Err(e) = res {
                eprintln!("{} {}", "Error:".red(), e);
            }
        }
    }
}
