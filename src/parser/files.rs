use std::path::PathBuf;
use walkdir::WalkDir;

use crate::{config::Configuration, log};

pub fn find_files(config: &Configuration) -> Vec<PathBuf> {
    return WalkDir::new(&config.input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|path| !is_excluded(path, &config.exclude))
        .collect();
}

fn is_excluded(path: &PathBuf, excluded: &[String]) -> bool {
    for dir in excluded {
        if path.components().any(|c| c.as_os_str() == dir.as_str()) {
            log::info(&format!("Analisando arquivo {:?}", path));
            return true;
        }
    }
    return false;
}
