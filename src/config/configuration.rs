use serde::Deserialize;
use std::fs;

use crate::log;

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
//TODO: implementar tela de configuração
impl Configuration {
    pub fn default() -> Self {
        log::info("Iniciando com a configuração padrão");
        return Self {
            input_path: "./src".into(),
            output_path: "./docs".into(),
            template_path: "../template.md".into(),
            exclude: vec![],
            recursive: false,
            include_hidden: false,
            verbose: true,
        };
    }

    pub fn from_file(path: Option<&str>) -> Self {
        let path = path.unwrap_or("Docs.toml");

        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };

        match toml::from_str(&contents) {
            Ok(config) => return config,
            Err(_) => return Self::default(),
        };
    }
}
