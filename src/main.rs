mod add;
mod list;
mod remove;
mod use_config;
mod user;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, styles = utils::get_clap_styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(add::AddArgs),
    Remove(remove::RemoveArgs),
    List(list::RemoveArgs),
    Use(use_config::UseArgs),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            add::add(args).await?;
        }
        Commands::Remove(args) => {
            remove::remove(args).await?;
        }
        Commands::List(_) => {
            list::list_users().await;
        }
        Commands::Use(args) => {
            use_config::use_config(args).await?;
        }
    }

    Ok(())
}
