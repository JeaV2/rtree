use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub mod config;
pub mod tree;

/// Lightweight info about a directory entry (avoids repeated `file_type()` calls).
#[derive(Debug, Clone, PartialEq)]
pub struct EntryInfo {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

/// Read, filter and sort entries for `path` according to `options`.
pub fn list_entries(
    path: &Path,
    options: &config::options::ResolvedOptions,
) -> io::Result<Vec<EntryInfo>> {
    let mut entries: Vec<EntryInfo> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name_os = e.file_name();
            let name = name_os.to_string_lossy().into_owned();
            let ft = e.file_type().ok()?;
            if !options.show_hidden && name.starts_with('.') {
                return None;
            }
            if options.only_dirs && !ft.is_dir() {
                return None;
            }
            Some(EntryInfo {
                name,
                path: e.path(),
                is_dir: ft.is_dir(),
            })
        })
        .collect();

    entries.sort_by_key(|a| a.name.to_lowercase());
    Ok(entries)
}

/// Render a single `EntryInfo` into the final printed string.
pub fn render_entry(
    prefix: &str,
    connector: &str,
    entry: &EntryInfo,
    options: &config::options::ResolvedOptions,
    parent: &Path,
) -> String {
    let file_color = if entry.is_dir {
        tree::color_to_ansi(&options.dir_color)
    } else {
        tree::color_to_ansi(&options.file_color)
    };
    tree::build_string(
        prefix,
        connector,
        file_color,
        entry.name.to_string(),
        options.no_markup,
        options.clickable,
        parent,
    )
}
