use crate::user::{User, UserOperator};
use crate::utils::paint_yellow;
use clap::Args;

/// add user config
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct AddArgs {
    pub alias: String,
    pub name: String,
    pub email: String,
}

pub async fn add(args: &AddArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut user_operator = UserOperator::new().await;
    user_operator
        .set_user(User {
            alias: args.alias.clone(),
            name: args.name.clone(),
            email: args.email.clone(),
        })
        .await?;

    println!("{} added successfully", paint_yellow(args.alias.as_str()),);
    Ok(())
}
