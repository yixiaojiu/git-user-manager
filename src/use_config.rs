use crate::user::UserOperator;
use crate::utils::{handle_command_error, paint_yellow};
use clap::Args;
use tokio::process::Command;

/// use alias to change git user config
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct UseArgs {
    pub alias: String,

    /// set local config, default value
    #[arg(short, long)]
    pub local: bool,

    /// set global config
    #[arg(short, long)]
    pub global: bool,
}

pub async fn use_config(
    args: &UseArgs,
    user_operator: &UserOperator,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = match user_operator.get_user(&args.alias) {
        Some(user) => user,
        None => {
            println!("{} not found", paint_yellow(args.alias.as_str()));
            return Ok(());
        }
    };

    let flag = if args.local {
        "--local"
    } else if args.global {
        "--global"
    } else {
        "--local"
    };

    let set_name_args = ["config", flag, "user.name", &user.name];
    let set_email_args = ["config", flag, "user.email", &user.email];

    let name_command_output = Command::new("git").args(set_name_args).output().await?;
    let email_command_output = Command::new("git").args(set_email_args).output().await?;

    if name_command_output.status.success() && email_command_output.status.success() {
        println!();
        println!(
            "{} is successfully used {}ly",
            paint_yellow(args.alias.as_str()),
            &flag[2..]
        );
    } else {
        handle_command_error(&name_command_output);
        handle_command_error(&email_command_output);
    }

    Ok(())
}
