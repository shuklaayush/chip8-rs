use clap::Parser;
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
}
