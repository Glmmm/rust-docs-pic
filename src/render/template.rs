use crate::config::Configuration;
use crate::log;
use crate::parser::TemplateData;
use handlebars::Handlebars;

use std::fs::{self, File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn generate_markdown(
    data: &TemplateData,
    config: &Configuration,
    original_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();

    let template_str = match fs::read_to_string(&config.template_path) {
        Ok(str) => str,
        Err(_) => {
            panic!(
                "Template não encontrado em '{}', impossível prosseguir",
                &config.template_path
            )
        }
    };

    log::info(
        &format!("Renderizando: {:?}", original_path),
        config.verbose,
    );

    handlebars.register_template_string("docs_template", template_str)?;

    let rendered = handlebars.render("docs_template", data)?;

    let relative_path = original_path.strip_prefix(&config.input_path)?;
    let mut target_path = PathBuf::from(&config.output_path).join(relative_path);
    target_path.set_extension("md");

    if let Some(parent) = target_path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(target_path)?;
    file.write_all(rendered.as_bytes())?;

    Ok(())
}
