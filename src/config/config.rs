use crate::config::utils::search_config_file;

#[derive(Debug, serde::Deserialize)]
pub struct Options {
    pub root_dir: String,
    pub exclude_patterns: Vec<String>,
    pub recursive: bool,
    pub show_hidden: bool,
}

impl Options {
    fn new() -> Self {
        Self {
            root_dir: ".".to_string(),
            exclude_patterns: vec![],
            recursive: true,
            show_hidden: false,
        }
    }

    pub fn from() -> Self {
        let config = search_config_file(&Self::new()).unwrap_or_else(|| {
            println!("Configurações não encontradas. Iniciando com valores padrão");
            Self::new()
        });
        Self {
            root_dir: config.root_dir,
            exclude_patterns: config.exclude_patterns,
            recursive: config.recursive,
            show_hidden: config.show_hidden,
        }
    }
}
