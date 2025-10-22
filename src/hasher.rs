use std::path::{Path, PathBuf};
use std::collections::HashMap;

// Given a vector of file paths, function generates a hash map (k: file_size, v: Vec<file_paths>)
// This is done to not waste time hashing files that have no way to have any duplicates
// If a file shares its size with no other files then it must be unique
pub fn pre_filter(files: Vec<PathBuf>) -> HashMap<u64, Vec<PathBuf>> {
    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for entry in files{
        if let Ok(metadata) = entry.metadata() {
            let size = metadata.len();
            size_map.entry(size).or_default().push(entry);
        }else{
            println!("Failed to get metadata for {:?}. File skipped", entry);
        }
    }
    size_map
}

pub fn hasher(size_map: &HashMap<u64, Vec<PathBuf>>) {

}