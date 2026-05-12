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
            clickable: false,
            no_markup: false,
        }
    }
}
// Combined resolved options from command-line arguments and config file.
pub struct ResolvedOptions {
    pub show_hidden: bool,
    pub only_dirs: bool,
    pub clickable: bool,
    pub no_markup: bool,
    pub dir_color: String,
    pub file_color: String,
}

impl ResolvedOptions {
    pub fn from_args_and_cfg(args: &config::Args, cfg: &config::options::Config) -> Self {
        let dir_color = args
            .dir_color
            .clone()
            .or_else(|| cfg.dir_color.clone())
            .unwrap_or_else(|| "blue".to_string());
        let file_color = args
            .file_color
            .clone()
            .or_else(|| cfg.file_color.clone())
            .unwrap_or_else(|| "green".to_string());

        ResolvedOptions {
            show_hidden: args.show_hidden || cfg.show_hidden,
            only_dirs: args.only_dirs || cfg.only_dirs,
            clickable: args.clickable || cfg.clickable,
            no_markup: args.no_markup || cfg.no_markup,
            dir_color,
            file_color,
        }
    }
}
