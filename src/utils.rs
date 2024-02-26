use clap::builder::{styling::AnsiColor, Styles};

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
