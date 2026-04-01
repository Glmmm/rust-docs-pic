use super::configuration::Configuration;
use walkdir::WalkDir;

pub fn search_config_file(options: &Configuration) -> Option<Configuration> {
    let walkdir = WalkDir::new(&options.root_dir).into_iter();
    for entry in walkdir.filter_map(|e| e.ok()) {
        if entry.file_name() == "Docs.toml" {
            let content = std::fs::read_to_string(entry.path()).ok()?;
            let config: Configuration = toml::from_str(&content).ok()?;
            println!("{:?}", config);
            return Some(config);
        }
    }
    None
}
