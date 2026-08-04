use clap::{Parser, error::Result};
use std::path::PathBuf;

use crate::errors::Errcode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub debug: bool,
    #[arg(short, long)]
    pub command: String,
    #[arg(short, long)]
    pub uid: u32,

    #[arg(short = 'm', long = "mount", value_parser = validate_mount_path)]
    pub mount_dir: PathBuf,
}

pub fn setup_logger(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .filter(None, level)
        .init();
}

fn validate_mount_path(s: &str) -> Result<PathBuf, Errcode> {
    let path = PathBuf::from(s);
    if !path.exists() || !path.is_dir() {
        return Err(Errcode::ArgumentInvalid("mount"));
    }
    Ok(path)
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::parse();

    if args.debug {
        setup_logger(log::LevelFilter::Debug);
    } else {
        setup_logger(log::LevelFilter::Info);
    };

    Ok(args)
}
