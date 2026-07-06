use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustdoc")]
#[command(about = "Analisa código Rust e gera documentação para SSG", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub config_path: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Rustdoc {
        #[arg(long)]
        rust: bool,
    },
}
