use std::path::PathBuf;
use clap::Parser;

use crate::config;

#[derive(Parser)]
#[command(version = config::LONG_VERSION, about, long_about = None)]
pub struct Args {
    #[arg(
        value_name = "PATH",
        default_value = ".",
        help = "The directory path to display"
    )]
    pub path: PathBuf,

    #[arg(short, long, help = "Include hidden files and directories")]
    pub show_hidden: bool,

    #[arg(short, long, help = "Show only directories")]
    pub only_dirs: bool,

    #[arg(
        short,
        long,
        help = "Set directory color (e.g., \"black\", \"red\", \"green\", \"yellow\", \"blue\", \"magenta/purple\", \"cyan\", \"white\")"
    )]
    pub dir_color: Option<String>,

    #[arg(
        short,
        long,
        help = "Set file color (e.g., \"black\", \"red\", \"green\", \"yellow\", \"blue\", \"magenta/purple\", \"cyan\", \"white\")"
    )]
    pub file_color: Option<String>,
}