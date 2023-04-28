#![allow(unused)]
#[macro_use]
extern crate log;


use elpis::tracing;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tracing::debug!("Booting application..");    
    Ok(())
}

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about=env!("CARGO_PKG_DESCRIPTION"),
    author=env!("CARGO_PKG_AUTHORS"),
    name=env!("CARGO_PKG_NAME"),
    version=env!("APP_VERSION"))]
pub struct Cli {
    #[arg(
        short = 'L',
        long,
        value_name = "LEVEL",
        default_value = "INFO",
        long_help = "",
    )]
    pub log_level: String,
    #[arg(
        short = 'D',
        long,
        long_help = "*DO NOT USE THIS OPTION UNLESS YOU KNOW WHAT YOU ARE DOING*"
    )]
    pub dev_mode: bool,
    #[arg(short, long)]
    pub gui: bool,
}
