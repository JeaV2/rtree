use clap::Parser;
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

fn visit_dir(path: &Path, depth: usize, arguments: &Args) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let name = entry.file_name();

        if !arguments.show_hidden && name.to_string_lossy().starts_with(".") {
            continue;
        }

        if entry_path.is_dir() && fs::read_dir(&entry_path).is_err() {
            continue;
        }

        println!("├─{}{}", "─".repeat(depth), name.to_string_lossy());

        if entry_path.is_dir() {
            unsafe { DIRS += 1 };
            visit_dir(&entry_path, depth + 1, arguments)?;
        }
        if entry_path.is_file() {
            unsafe { FILES += 1 };
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!(".");
    visit_dir(Path::new(&args.path), 0, &args)?;
    println!("\n{} directories, {} files", unsafe { DIRS }, unsafe { FILES });
    Ok(())
}
