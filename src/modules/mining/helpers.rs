use super::types::*;

pub fn bar(current: f64, max: f64, width: usize) -> String {
    let filled = ((current / max) * width as f64).round() as usize;
    let filled = filled.min(width);
    format!("[{}{}]", "▰".repeat(filled), "▱".repeat(width - filled))
}