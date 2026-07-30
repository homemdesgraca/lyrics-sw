use crate::declarations::*;

use std::fmt::Display;

use owo_colors::{OwoColorize};

pub fn log_error(text: impl Display) {
    eprintln!("{} {}", "ERROR:".style(ERROR_STYLE_BOLD), text.style(ERROR_STYLE))
}

pub fn log_warn(text: impl Display) {
    eprintln!("{} {}", "WARN:".style(WARN_STYLE_BOLD), text.style(WARN_STYLE))
}

pub fn log_info(text: impl Display) {
    println!("{} {}", "INFO:".style(INFO_STYLE_BOLD), text.style(INFO_STYLE))
}

pub fn line_break() {
    let line_break = "---------------".style(LINE_BREAK);
    println!("{}", line_break)
}