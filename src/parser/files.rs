use std::path::PathBuf;
use walkdir::WalkDir;

use crate::config::Configuration;

pub fn find_files(config: &Configuration) -> Vec<PathBuf> {
    return WalkDir::new(&config.input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|path| !is_excluded(path, &config.exclude))
        .collect();
}

pub fn search_config_file(config: &Configuration) -> Option<Configuration> {
    let walkdir = WalkDir::new(&config.input_path).into_iter();
    for entry in walkdir.filter_map(|e| e.ok()) {
        if entry.file_name() == "Docs.toml" {
            let content = std::fs::read_to_string(entry.path()).ok()?;
            let config: Configuration = toml::from_str(&content).ok()?;
            println!("{:?}", &config);
            return Some(config);
        }
    }
    None
}

fn is_excluded(path: &PathBuf, excluded: &[String]) -> bool {
    for dir in excluded {
        if path.components().any(|c| c.as_os_str() == dir.as_str()) {
            return true;
        }
    }
    return false;
}
