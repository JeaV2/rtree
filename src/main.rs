use std::fs;
use std::io;
use std::path::Path;

mod config;

fn visit_dir(path: &Path, depth: usize) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let name = entry.file_name();

        println!("├{}{}", "─".repeat(depth), name.to_string_lossy());

        if entry_path.is_dir() {
            visit_dir(&entry_path, depth + 1)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = config::get_args();
    let path = if args.len() > 1 { &args[1] } else { "/" };

    println!(".");
    visit_dir(Path::new(path), 0)
}