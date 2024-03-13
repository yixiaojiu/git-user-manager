use crate::{
    user::UserOperator,
    utils::{paint_green, paint_yellow},
};
use clap::Args;

/// list all user config
#[derive(Args, Debug)]
#[command(alias = "ls")]
#[command(version, about, long_about = None)]
pub struct RemoveArgs {}

pub async fn list_users(user_operator: &UserOperator) {
    let users = user_operator.get_users();

    println!();
    if users.len() == 0 {
        println!("{}", 
        paint_yellow("It seems that there is no available configuration. Use the `gum add` to add the configuration."));
        return;
    }

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
    println!("{}", output);
}
