use crate::user::UserOperator;
use crate::utils::{paint_green, paint_yellow};
use clap::Args;

/// use alias to remove a user config
#[derive(Args, Debug)]
#[command(version, about, alias = "rm", long_about = None)]
pub struct RemoveArgs {
    /// config alias
    pub alias: Option<String>,

    /// remove all users
    #[arg(short, long, default_value = "false")]
    pub all: bool,
}

pub fn remove(args: &RemoveArgs, user_operator: &mut UserOperator) {
    println!();
    if args.all {
        user_operator.remove_all();
        println!("{}", paint_green("All users removed successfully"));
        return;
    }

    if let Some(alias) = &args.alias {
        user_operator.remove_user(alias.as_str());
        println!("{} removed successfully", paint_yellow(alias.as_str()));
    }
}
