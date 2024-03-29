use std::path::{Path, PathBuf};


pub fn find_directory(root_dir: &Path, target_dir_name: &str) -> Option<PathBuf> {
    if root_dir.is_dir() {
        // Check if the current directory contains the target directory
        if let Some(dir_entry) = root_dir
            .read_dir()
            .ok()?
            .find(|entry| {
                entry.as_ref()
                    .ok()
                    .map_or(false, |e| e.file_name().to_string_lossy() == target_dir_name)
            })
        {
            if let Ok(dir_entry) = dir_entry {
                let dir_path = dir_entry.path();
                if dir_path.is_dir() {
                    return Some(dir_path);
                }
            }
        }
        // Recursively search subdirectories
        for entry in root_dir.read_dir().ok()? {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(found_path) = find_directory(&path, target_dir_name) {
                    return Some(found_path);
                }
            }
        }
    }
    None
}