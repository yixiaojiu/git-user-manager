use clap::builder::{styling::AnsiColor, Styles};
use dirs::data_dir;
use tokio::fs::create_dir_all;

pub fn get_clap_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

pub fn paint_green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

#[allow(dead_code)]
pub fn paint_yellow(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}

pub async fn create_data_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut path_buf = data_dir().unwrap();
    path_buf.push("gum");

    if !path_buf.exists() {
        create_dir_all(&path_buf).await.unwrap();
    }

    Ok(path_buf)
}
