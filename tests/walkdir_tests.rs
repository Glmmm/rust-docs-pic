#[cfg(test)]
mod walkdir_tests {
    use rust_docs_pic::{config::config::Options, list_directory};

    #[test]
    fn test_list_directory() {
        let config = Options::from();

        list_directory(&config);
    }
    #[test]
    fn test_config_from_file() {
        let config = Options::from();
        assert_eq!(config.root_dir, ".");
        assert_eq!(config.exclude_patterns, Vec::<String>::new());
        assert!(config.recursive);
        assert!(!config.show_hidden);
    }
}
