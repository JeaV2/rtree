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
    // The path to the file to read
    path: PathBuf,
    // Whether to show hidden files
    #[arg(short, long)]
    show_hidden: bool,
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
        })
        .collect();

    for (idx, entry) in entries.iter().enumerate() {
        // Get the path and name of the entry
        let entry_path = entry.path();
        let name = entry.file_name();
        // Determine if this is the last entry in the directory
        let is_last_entry = idx == entries.len() - 1;

        // Tree characters
        let connector = if is_last_entry {
            "└──"
        } else {
            "├──"
        };
        let next_prefix = if is_last_entry { "    " } else { "│   " };

        // Print the entry name with the appropriate prefix and connector
        println!("{}{}{}", prefix, connector, name.to_string_lossy());

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
