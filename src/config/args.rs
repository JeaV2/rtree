use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::config;

#[derive(Parser)]
#[command(version = config::LONG_VERSION, about, long_about = None)]
#[command(
    after_help = "You can also set these options in a configuration file located at \x1b[40m`~/.config/rtree/rtree.toml`\x1b[0m. Command-line arguments will override the configuration file settings."
)]
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

    #[arg(
        short,
        long,
        help = "Makes it so files are files and directories are clickable. When clicked, it will open the file or directory in the default file manager or associated application."
    )]
    pub clickable: bool,

    #[arg(
        short,
        long,
        help = "Disables all ANSI color codes and terminal links, resulting in plain text output."
    )]
    pub no_markup: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dir_color: Option<String>,
    pub file_color: Option<String>,
    pub show_hidden: bool,
    pub only_dirs: bool,
    pub clickable: bool,
    pub no_markup: bool,
}
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            dir_color: Some("blue".to_string()),
            file_color: Some("green".to_string()),
            show_hidden: false,
            only_dirs: false,
            clickable: true,
            no_markup: false,
        }
    }
}
