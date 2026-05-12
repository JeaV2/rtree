use rtree::{config::options::ResolvedOptions, list_entries};
use std::fs;
use tempfile::TempDir;

fn create_test_dir() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = temp_dir.path();

    // Create test files and directories
    fs::write(base.join("file_b.txt"), "content").expect("Failed to create file_b.txt");
    fs::write(base.join("file_a.txt"), "content").expect("Failed to create file_a.txt");
    fs::create_dir(base.join("dir_b")).expect("Failed to create dir_b");
    fs::create_dir(base.join("dir_a")).expect("Failed to create dir_a");
    fs::write(base.join(".hidden_file"), "content").expect("Failed to create .hidden_file");
    fs::create_dir(base.join(".hidden_dir")).expect("Failed to create .hidden_dir");

    temp_dir
}

#[test]
fn test_list_entries_without_hidden_files() {
    let temp_dir = create_test_dir();
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: false,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };

    let entries = list_entries(temp_dir.path(), &options).expect("Failed to list entries");

    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    assert_eq!(names.len(), 4);
    assert!(!names.contains(&".hidden_file".to_string()));
    assert!(!names.contains(&".hidden_dir".to_string()));
}

#[test]
fn test_list_entries_with_hidden_files() {
    let temp_dir = create_test_dir();
    let options = ResolvedOptions {
        show_hidden: true,
        only_dirs: false,
        clickable: false,
        no_markup: false,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };

    let entries = list_entries(temp_dir.path(), &options).expect("Failed to list entries");

    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    assert_eq!(names.len(), 6);
    assert!(names.contains(&".hidden_file".to_string()));
    assert!(names.contains(&".hidden_dir".to_string()));
}

#[test]
fn test_list_entries_only_dirs() {
    let temp_dir = create_test_dir();
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: true,
        clickable: false,
        no_markup: false,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };

    let entries = list_entries(temp_dir.path(), &options).expect("Failed to list entries");

    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"dir_a".to_string()));
    assert!(names.contains(&"dir_b".to_string()));
    assert!(!names.contains(&"file_a.txt".to_string()));
    assert!(!names.contains(&"file_b.txt".to_string()));
}

#[test]
fn test_list_entries_sorted_case_insensitive() {
    let temp_dir = create_test_dir();
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: false,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };

    let entries = list_entries(temp_dir.path(), &options).expect("Failed to list entries");

    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    // Should be sorted: dir_a, dir_b, file_a.txt, file_b.txt
    assert_eq!(names[0], "dir_a");
    assert_eq!(names[1], "dir_b");
    assert_eq!(names[2], "file_a.txt");
    assert_eq!(names[3], "file_b.txt");
}

#[test]
fn test_list_entries_is_dir_flag() {
    let temp_dir = create_test_dir();
    let options = ResolvedOptions {
        show_hidden: false,
        only_dirs: false,
        clickable: false,
        no_markup: false,
        dir_color: "blue".to_string(),
        file_color: "green".to_string(),
    };

    let entries = list_entries(temp_dir.path(), &options).expect("Failed to list entries");

    for entry in entries {
        if entry.name.contains("dir") {
            assert!(entry.is_dir, "Expected {} to be a directory", entry.name);
        } else {
            assert!(!entry.is_dir, "Expected {} to be a file", entry.name);
        }
    }
}
