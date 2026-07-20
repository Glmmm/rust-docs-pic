use chrono::Local;

pub fn error(text: &str) {
    let timestamp = Local::now();
    let formatted_time = timestamp.format("%H:%M:%S");
    println!("[ERROR {}] {}", formatted_time, text);
}

pub fn warn(text: &str) {
    let timestamp = Local::now();
    let formatted_time = timestamp.format("%H:%M:%S");
    println!("[WARN {}] {}", formatted_time, text)
}
pub fn info(text: &str) {
    let timestamp = Local::now();
    let formatted_time = timestamp.format("%H:%M:%S");
    println!("[INFO {}] {}", formatted_time, text)
}
