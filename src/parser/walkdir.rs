use walkdir::{DirEntry, WalkDir};

use crate::config::configuration::Configuration;

pub fn list_directory(config: &Configuration) {
    let walker = WalkDir::new(&config.root_dir).into_iter();
    walker
        .filter_entry(|e| !is_excluded(e, config))
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            println!("{}", entry.path().display());
        });
}

fn is_excluded(entry: &DirEntry, config: &Configuration) -> bool {
    if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
        if config
            .exclude_patterns
            .iter()
            .any(|pattern| pattern == name)
        {
            return true;
        }
    }
    false
}
