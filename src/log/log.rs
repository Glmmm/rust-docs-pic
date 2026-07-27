use chrono::Local;
use colored::*;

#[derive(Copy, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub fn log(level: LogLevel, text: &str, verbose: bool) {
    if !verbose {
        return;
    }

    let formatted_time = Local::now().format("%H:%M:%S");

    let prefix = match level {
        LogLevel::Info => format!("[INFO {}]", formatted_time).blue().bold(),
        LogLevel::Warn => format!("[WARN {}]", formatted_time).yellow().bold(),
        LogLevel::Error => format!("[ERROR {}]", formatted_time).red().bold(),
    };

    println!("{} {}", prefix, text);
}

pub fn info(text: &str, verbose: bool) {
    log(LogLevel::Info, text, verbose);
}

pub fn warn(text: &str, verbose: bool) {
    log(LogLevel::Warn, text, verbose);
}

pub fn error(text: &str, verbose: bool) {
    log(LogLevel::Error, text, verbose);
}
