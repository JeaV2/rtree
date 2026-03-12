use std::fs;
use std::io;
mod config;

fn main() -> io::Result<()> {
    let args = config::get_args();
    let path = if args.len() > 1 { &args[1] } else { "/" };
    let files = fs::read_dir(path)?;
    for file in files {
        let file = file?;
        let filename = file.file_name();
        println!("{} D:{}", filename.to_string_lossy(), file.file_type()?.is_dir());
    }
    Ok(())
}