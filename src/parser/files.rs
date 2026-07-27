use std::path::PathBuf;

use walkdir::WalkDir;

use crate::config::Configuration;

pub fn find_files(config: &Configuration) -> Vec<PathBuf> {
    return WalkDir::new(&config.input_path)
        .max_depth(config.depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|path| !is_excluded(path, &config))
        .collect();
}

fn is_excluded(path: &PathBuf, config: &Configuration) -> bool {
    for dir in &config.exclude {
        if path.components().any(|c| c.as_os_str() == dir.as_str()) {
            return true;
        }
    }
    return false;
}
