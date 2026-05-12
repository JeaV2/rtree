use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

use rtree::{config, list_entries, render_entry};

// Traverse directory `path` and print tree using `options`.
fn traverse(
    path: &Path,
    prefix: &str,
    options: &config::options::ResolvedOptions,
    visited: &mut HashSet<std::path::PathBuf>,
    files_dirs: &mut (usize, usize),
) -> io::Result<()> {
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

    let entries = list_entries(path, options)?;
    for (idx, entry) in entries.iter().enumerate() {
        let is_last_entry = idx == entries.len() - 1;
        let connector = if is_last_entry {
            "└──"
        } else {
            "├──"
        };
        let next_prefix = if is_last_entry { "    " } else { "│   " };

        println!("{}", render_entry(prefix, connector, entry, options, path));

        if entry.is_dir {
            if fs::read_dir(&entry.path).is_ok() {
                files_dirs.1 += 1;
                let new_prefix = format!("{}{}", prefix, next_prefix);
                traverse(&entry.path, &new_prefix, options, visited, files_dirs)?;
            }
        } else {
            files_dirs.0 += 1;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Parse command-line arguments and load config.
    let args = config::Args::parse();
    let cfg: config::options::Config = confy::load("rtree", "rtree").unwrap_or_default();
    let options = config::options::ResolvedOptions::from_args_and_cfg(&args, &cfg);

    // Initialize counters for files and directories
    let mut files_dirs: (usize, usize) = (0, 0);

    // Start visiting the directory and print the tree structure
    println!("{}:", args.path.display());
    let mut visited = HashSet::new();
    traverse(
        Path::new(&args.path),
        "",
        &options,
        &mut visited,
        &mut files_dirs,
    )?;
    print!("\n{} directories, {} files\n", files_dirs.1, files_dirs.0);
    Ok(())
}
