use rtree::{EntryInfo, config::options::ResolvedOptions, render_entry};
use std::path::PathBuf;

#[test]
fn test_render_entry_file_with_color() {
    let entry = EntryInfo {
        name: "test_file.txt".to_string(),
        path: PathBuf::from("/path/to/test_file.txt"),
        is_dir: false,
    };
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: false,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };
    let parent = std::path::Path::new("/path/to");

    let result = render_entry("", "├── ", &entry, &options, parent);

    assert!(result.contains("├── "));
    assert!(result.contains("test_file.txt"));
    // Should contain ANSI codes for green color (for files)
    assert!(result.contains("\x1b[92m"));
}

#[test]
fn test_render_entry_directory_with_color() {
    let entry = EntryInfo {
        name: "test_dir".to_string(),
        path: PathBuf::from("/path/to/test_dir"),
        is_dir: true,
    };
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: false,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };
    let parent = std::path::Path::new("/path/to");

    let result = render_entry("", "└── ", &entry, &options, parent);

    assert!(result.contains("└── "));
    assert!(result.contains("test_dir"));
    // Should contain ANSI codes for blue color (for directories)
    assert!(result.contains("\x1b[94m"));
}

#[test]
fn test_render_entry_no_markup() {
    let entry = EntryInfo {
        name: "test_file.txt".to_string(),
        path: PathBuf::from("/path/to/test_file.txt"),
        is_dir: false,
    };
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: true,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };
    let parent = std::path::Path::new("/path/to");

    let result = render_entry("  ", "├── ", &entry, &options, parent);

    // Should have prefix and connector
    assert!(result.starts_with("  ├── "));
    assert!(result.contains("test_file.txt"));
    // Should NOT contain ANSI escape codes
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_render_entry_with_prefix() {
    let entry = EntryInfo {
        name: "file.txt".to_string(),
        path: PathBuf::from("/path/to/file.txt"),
        is_dir: false,
    };
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: true,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };
    let parent = std::path::Path::new("/path/to");
    let prefix = "│   ";

    let result = render_entry(prefix, "├── ", &entry, &options, parent);

    assert!(result.starts_with("│   ├── "));
}

#[test]
fn test_render_entry_last_entry_connector() {
    let entry = EntryInfo {
        name: "last_file.txt".to_string(),
        path: PathBuf::from("/path/to/last_file.txt"),
        is_dir: false,
    };
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: true,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };
    let parent = std::path::Path::new("/path/to");

    let result = render_entry("", "└── ", &entry, &options, parent);

    assert!(result.contains("└── last_file.txt"));
}
