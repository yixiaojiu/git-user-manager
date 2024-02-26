use crate::user::UserOperator;
use crate::utils::paint_yellow;
use clap::Args;

/// use alias to remove a user config
#[derive(Args, Debug)]
#[command(version, about, alias = "rm", long_about = None)]
pub struct RemoveArgs {
    pub alias: String,
}

pub async fn remove(args: &RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut user_operator = UserOperator::new().await;
    user_operator.remove_user(&args.alias).await?;
    println!("{} removed successfully", paint_yellow(args.alias.as_str()));

    Ok(())
}
