use clap::Parser;
use ratatui::style::Color;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    #[arg(required = true, value_parser)]
    pub rom: PathBuf,

    #[arg(long = "clock-frequency", default_value_t = 560)]
    pub clk_freq: u64,
    #[arg(long, default_value_t = 60)]
    pub refresh_rate: u64,

    #[arg(long, default_value_t = false)]
    pub headless: bool,

    #[arg(long = "background-color", default_value_t = Color::Black)]
    pub bg_color: Color,
    #[arg(long = "foreground-color", default_value_t = Color::White)]
    pub fg_color: Color,
    #[arg(long = "border-color", default_value_t = Color::White)]
    pub border_color: Color,
}
