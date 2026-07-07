use clap::Parser;
use rustdoc::config::{Cli, Commands, Configuration};
use rustdoc::parser::{TemplateData, extract_ast, find_files};
use rustdoc::render::generate_markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = Configuration::from_file(cli.config_path.as_deref());

    match cli.command {
        Commands::Rustdoc { rust } => {
            if rust {
                run_analyzer(config)?;
            }
        }
    }
    Ok(())
}

fn run_analyzer(config: Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let files = find_files(&config);
    for path in files {
        if let Ok(source_code) = std::fs::read_to_string(&path) {
            if let Ok((structs, fields, functions)) = extract_ast(&source_code) {
                let data = TemplateData {
                    name: path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                    source_code,
                    structs,
                    fields,
                    functions,
                    path: path.to_string_lossy().into_owned(),
                };

                generate_markdown(&data, &config, &path)?;
            } else {
                break;
            }
        }
    }
    Ok(())
}
