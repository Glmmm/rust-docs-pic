use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub input_path: String,
    pub output_path: String,
    pub template_path: String,
    pub exclude: Vec<String>,
    pub recursive: bool,
    pub include_hidden: bool,
    pub verbose: bool,
}

impl Configuration {
    pub fn default() -> Self {
        return Self {
            input_path: "./src".into(),
            output_path: "docs".into(),
            template_path: "../template.md".into(),
            exclude: vec![
                [
                    "client",
                    "target",
                    "tests",
                    "examples",
                    "template.md",
                    "docs",
                    "node_modules",
                    "build.rs",
                ]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            ],
            recursive: false,
            include_hidden: false,
            verbose: true,
        };
    }

    pub fn from_file(path: Option<&str>) -> Self {
        let path = path.unwrap_or_else(|| ".");

        let file = &fs::read_to_string(path).unwrap_or_else(|_| String::new());

        let toml: Self = match toml::from_str(file) {
            Ok(config) => config,
            Err(_) => return Self::default(),
        };
        return toml;
    }
}
