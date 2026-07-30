use crate::declarations::*;

use std::fmt::{Debug, Display};

use owo_colors::{OwoColorize};

pub fn log_error_display(text: impl Display) {
    eprintln!("{} {}", "ERROR:".style(ERROR_STYLE_BOLD), text.style(ERROR_STYLE))
}

pub fn log_error(text: impl Debug) {
    println!("{} {:#?}", "ERROR:".style(ERROR_STYLE_BOLD), text.style(ERROR_STYLE))
}

pub fn log_warn(text: impl Display) {
    eprintln!("{} {}", "WARN:".style(WARN_STYLE_BOLD), text.style(WARN_STYLE))
}

pub fn log_info(text: impl Display) {
    println!("{} {}", "INFO:".style(INFO_STYLE_BOLD), text.style(INFO_STYLE))
}

pub fn log_debug_display(text: impl Display) {
    if is_debug_enabled() {
    println!("{} {}", "DEBUG:".style(DEBUG_STYLE_BOLD), text.style(DEBUG_STYLE))
    }
}

pub fn line_break() {
    let line_break = "---------------".style(LINE_BREAK);
    println!("{}", line_break)
}