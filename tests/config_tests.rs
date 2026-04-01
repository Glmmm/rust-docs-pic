#[cfg(test)]
mod config_tests {
    use rust_docs_pic::{config::configuration::Configuration, parser::walkdir::list_directory};

    #[test]
    fn test_list_directory() {
        let config = Configuration::from_file();

        list_directory(&config);
    }
}
