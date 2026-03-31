use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

mod config;

static mut FILES: usize = 0;
static mut DIRS: usize = 0;

#[derive(Parser)]
#[command(version = config::LONG_VERSION, about, long_about = None)]
struct Args {
    #[arg(
        value_name = "PATH",
        default_value = ".",
        help = "The directory path to display"
    )]
    path: PathBuf,

    #[arg(short, long, help = "Include hidden files and directories")]
    show_hidden: bool,

    #[arg(short, long, help = "Show only directories")]
    only_dirs: bool,

    #[arg(
        short,
        long,
        help = "Set directory color (e.g., \"black\", \"red\", \"green\", \"yellow\", \"blue\", \"magenta/purple\", \"cyan\", \"white\")"
    )]
    dir_color: Option<String>,

    #[arg(
        short,
        long,
        help = "Set file color (e.g., \"black\", \"red\", \"green\", \"yellow\", \"blue\", \"magenta/purple\", \"cyan\", \"white\")"
    )]
    file_color: Option<String>,
}

fn visit_dir(
    path: &Path,
    prefix: &str,
    _is_last: bool,
    arguments: &Args,
    visited: &mut HashSet<PathBuf>,
) -> io::Result<()> {
    // Canonicalize and check if already visited
    let canonical_path = match fs::canonicalize(path) {
        Ok(p) => p,
        Err(_) => return Ok(()),
    };
    if visited.contains(&canonical_path) {
        return Ok(());
    }
    visited.insert(canonical_path);

    // Read the directory entries and filter them based on the arguments
    let entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            !(!arguments.show_hidden && name.to_string_lossy().starts_with("."))
                && !(arguments.only_dirs && !e.path().is_dir())
        })
        .collect();

    for (idx, entry) in entries.iter().enumerate() {
        // Get the path and name of the entry
        let entry_path = entry.path();
        let name = entry.file_name();
        // Determine if this is the last entry in the directory
        let is_last_entry = idx == entries.len() - 1;

        let file_color = if entry_path.is_dir() {
            config::color_to_ansi(arguments.dir_color.as_ref().map_or("blue", |s| s.as_str()))
        } else {
            config::color_to_ansi(
                arguments
                    .file_color
                    .as_ref()
                    .map_or("green", |s| s.as_str()),
            )
        };

        // Tree characters
        let connector = if is_last_entry {
            "└──"
        } else {
            "├──"
        };
        let next_prefix = if is_last_entry { "    " } else { "│   " };

        // Print the entry name with the appropriate prefix and connector
        println!(
            "{}{}{}{}\x1b[0m",
            prefix,
            connector,
            file_color,
            name.to_string_lossy()
        );

        // If the entry is a directory, recursively visit it; if it's a file, count it
        if entry_path.is_dir() && fs::read_dir(&entry_path).is_ok() {
            unsafe { DIRS += 1 };
            let new_prefix = format!("{}{}", prefix, next_prefix);
            visit_dir(&entry_path, &new_prefix, is_last_entry, arguments, visited)?;
        } else if entry_path.is_file() {
            unsafe { FILES += 1 };
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();
    // Start visiting the directory and print the tree structure
    println!("{}:", args.path.display());
    let mut visited = HashSet::new();
    visit_dir(Path::new(&args.path), "", true, &args, &mut visited)?;
    println!("\n{} directories, {} files", unsafe { DIRS }, unsafe {
        FILES
    });
    Ok(())
}
