use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    #[arg(required = true, value_parser)]
    pub rom: PathBuf,
    #[arg(default_value_t = 480)]
    pub clk_freq: u64,
    #[arg(default_value_t = 60)]
    pub fps: u64,
}
