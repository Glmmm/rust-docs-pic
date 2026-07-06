use crate::config::Configuration;
use crate::parser::TemplateData;
use handlebars::Handlebars;

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn generate_markdown(
    data: &TemplateData,
    config: &Configuration,
    original_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();

    let template_str = include_str!("../template.md");

    handlebars.register_template_string("doc_template", template_str)?;

    let rendered = handlebars.render("doc_template", data)?;

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
