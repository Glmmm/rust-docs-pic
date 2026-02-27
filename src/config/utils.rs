use walkdir::WalkDir;

use crate::config::config::Options;

pub fn search_config_file(options: &Options) -> Option<Options> {
    let walkdir = WalkDir::new(&options.root_dir).into_iter();
    for entry in walkdir.filter_map(|e| e.ok()) {
        if entry.file_name() == "docs_config.toml" {
            let content = std::fs::read_to_string(entry.path()).ok()?;
            let config: Options = toml::from_str(&content).ok()?;
            println!("{:?}", config);
            return Some(config);
        }
    }
    println!("erro {:?}", options);
    None
}
