use crate::user::UserOperator;
use crate::utils::create_data_dir;
use crate::utils::paint_yellow;
use clap::Args;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// conditional include, reference https://git-scm.com/docs/git-config#_conditional_includes
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct IncludeArgs {
    pub alias: String,

    /// gitdir
    pub pattern: String,

    #[arg(short, long)]
    pub remove: Option<String>,
}

pub async fn include(
    args: &IncludeArgs,
    user_operator: &mut UserOperator,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = match user_operator.get_user(&args.alias) {
        Some(user) => user,
        None => {
            println!("{} not found", paint_yellow(args.alias.as_str()));
            return Ok(());
        }
    };

    let config_content = format!("[user]\n  name = {}\n  email = {}", user.name, user.email);
    let config_file_name = format!("{}-include.toml", user.alias);
    let mut config_path = create_data_dir().await.unwrap();
    config_path.push(config_file_name);

    let mut file: File;
    if !config_path.exists() {
        file = File::create(&config_path).await.unwrap();
    } else {
        file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&config_path)
            .await
            .unwrap();
    }

    file.write_all(config_content.as_bytes()).await?;
    file.sync_data().await?;

    Command::new("git")
        .args([
            "config",
            "--global",
            format!("includeif.gitdir:{}", args.pattern).as_str(),
            config_path.display().to_string().as_str(),
        ])
        .output()
        .await?;

    Ok(())
}
