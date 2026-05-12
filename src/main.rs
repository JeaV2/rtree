use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Config {
    dir_color: Option<String>,
    file_color: Option<String>,
    show_hidden: bool,
    only_dirs: bool,
    clickable: bool,
    no_color: bool,
}
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            dir_color: Some("blue".to_string()),
            file_color: Some("green".to_string()),
            show_hidden: false,
            only_dirs: false,
            clickable: true,
            no_color: false,
        }
    }
}

mod config;

fn visit_dir(
    path: &Path,
    prefix: &str,
    _is_last: bool,
    arguments: &config::Args,
    visited: &mut HashSet<PathBuf>,
    files_dirs: &mut (usize, usize),
) -> io::Result<()> {
    // Load configuration file
    let cfg: Config = confy::load("rtree", "rtree").unwrap_or_default();
    // Resolve metadata without following symlinks.
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return Ok(()),
    };

    if !metadata.is_dir() {
        return Ok(());
    }

    // Canonicalize real directories for cycle detection only.
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
            let file_type = match e.file_type() {
                Ok(t) => t,
                Err(_) => return false,
            };
            let show_hidden = arguments.show_hidden || cfg.show_hidden;
            let only_dirs = arguments.only_dirs || cfg.only_dirs;
            !(!show_hidden && name.to_string_lossy().starts_with("."))
                && !(only_dirs && !file_type.is_dir())
        })
        .collect();

    for (idx, entry) in entries.iter().enumerate() {
        // Get the path and name of the entry
        let entry_path = entry.path();
        let name = entry.file_name();
        let entry_file_type = match entry.file_type() {
            Ok(m) => m,
            Err(_) => continue,
        };
        // Determine if this is the last entry in the directory
        let is_last_entry = idx == entries.len() - 1;

        let file_color = if entry_file_type.is_dir() {
            config::color_to_ansi(
                arguments
                    .dir_color
                    .as_ref()
                    .map_or(cfg.dir_color.as_deref().unwrap_or("blue"), |s| s.as_str()),
            )
        } else {
            config::color_to_ansi(
                arguments
                    .file_color
                    .as_ref()
                    .map_or(cfg.file_color.as_deref().unwrap_or("green"), |s| s.as_str()),
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
            "{}",
            config::build_string(prefix, connector, file_color, name.to_string_lossy(), arguments.no_color)
        );

        // If the entry is a directory, recursively visit it; if it's a file, count it
        if entry_file_type.is_dir() {
            if fs::read_dir(&entry_path).is_ok() {
                files_dirs.1 += 1;
                let new_prefix = format!("{}{}", prefix, next_prefix);
                visit_dir(
                    &entry_path,
                    &new_prefix,
                    is_last_entry,
                    arguments,
                    visited,
                    files_dirs,
                )?;
            }
        } else if entry_file_type.is_file() {
            files_dirs.0 += 1;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args = config::Args::parse();
    // Initialize counters for files and directories
    let mut files_dirs: (usize, usize) = (0, 0);
    // Start visiting the directory and print the tree structure
    println!("{}:", args.path.display());
    let mut visited = HashSet::new();
    visit_dir(
        Path::new(&args.path),
        "",
        true,
        &args,
        &mut visited,
        &mut files_dirs,
    )?;
    print!("\n{} directories, {} files\n", files_dirs.1, files_dirs.0);
    Ok(())
}
