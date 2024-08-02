use crate::utils::{handle_command_error, paint_yellow};
use clap::Args;
use tokio::process::Command;

/// View git username and email configuration
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct InfoArgs {
    /// show local config, default value
    #[arg(short, long)]
    pub local: bool,

    /// show global config
    #[arg(short, long)]
    pub global: bool,
}

pub async fn info(args: &InfoArgs) -> Result<(), Box<dyn std::error::Error>> {
    let flag = if args.local {
        "--local"
    } else if args.global {
        "--global"
    } else {
        "--local"
    };

    let info_name_args = ["config", flag, "user.name"];
    let info_email_args = ["config", flag, "user.email"];

    let name_command_output = Command::new("git").args(info_name_args).output().await?;
    let email_command_output = Command::new("git").args(info_email_args).output().await?;

    if name_command_output.status.success() && email_command_output.status.success() {
        let username = String::from_utf8_lossy(&name_command_output.stdout);
        let email = String::from_utf8_lossy(&email_command_output.stdout);

        println!();
        println!(
            "Current {} config: {}",
            &flag[2..],
            paint_yellow(format!("{}<{}>", username.trim(), email.trim()).as_str())
        );
    } else {
        handle_command_error(&name_command_output);
        handle_command_error(&email_command_output);
    }

    return Ok(());
}
