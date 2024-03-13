mod add;
mod list;
mod remove;
mod use_config;
mod user;
mod utils;

use clap::{Parser, Subcommand};
use user::UserOperator;

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
    let mut user_operator = UserOperator::new().await;

    match &cli.command {
        Commands::Add(args) => {
            add::add(args, &mut user_operator).await?;
        }
        Commands::Remove(args) => {
            remove::remove(args, &mut user_operator).await?;
        }
        Commands::List(_) => {
            list::list_users(&user_operator).await;
        }
        Commands::Use(args) => {
            use_config::use_config(args, &user_operator).await?;
        }
    }

    user_operator.sync_config().await?;

    Ok(())
}
