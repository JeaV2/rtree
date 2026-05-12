use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

mod config;
mod tree;

fn visit_dir(
    path: &Path,
    prefix: &str,
    arguments: &config::Args,
    visited: &mut HashSet<PathBuf>,
    files_dirs: &mut (usize, usize),
) -> io::Result<()> {
    // Load configuration file
    let cfg: config::args::Config = confy::load("rtree", "rtree").unwrap_or_default();
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
            tree::color_to_ansi(
                arguments
                    .dir_color
                    .as_ref()
                    .map_or(cfg.dir_color.as_deref().unwrap_or("blue"), |s| s.as_str()),
            )
        } else {
            tree::color_to_ansi(
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
        let clickable = arguments.clickable || cfg.clickable;
        let no_markup = arguments.no_markup || cfg.no_markup;
        println!(
            "{}",
            tree::build_string(
                prefix,
                connector,
                file_color,
                name.to_string_lossy(),
                no_markup,
                clickable,
                path
            )
        );

        // If the entry is a directory, recursively visit it; if it's a file, count it
        if entry_file_type.is_dir() {
            if fs::read_dir(&entry_path).is_ok() {
                files_dirs.1 += 1;
                let new_prefix = format!("{}{}", prefix, next_prefix);
                visit_dir(
                    &entry_path,
                    &new_prefix,
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
        &args,
        &mut visited,
        &mut files_dirs,
    )?;
    print!("\n{} directories, {} files\n", files_dirs.1, files_dirs.0);
    Ok(())
}
