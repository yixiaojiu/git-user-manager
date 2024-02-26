use crate::{user::UserOperator, utils::paint_green};
use clap::Args;

/// list all user config
#[derive(Args, Debug)]
#[command(alias = "ls")]
#[command(version, about, long_about = None)]
pub struct RemoveArgs {}

pub async fn list_users() {
    let user_operator = UserOperator::new().await;
    let users = user_operator.get_users();

    let output = users
        .iter()
        .map(|user| {
            format!(
                "  {:24} {}<{}>",
                paint_green(user.alias.as_str()),
                user.name,
                user.email
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!();
    println!("{}", output);
}
