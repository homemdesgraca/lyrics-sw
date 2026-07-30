use serde::Deserialize;
use owo_colors::{Style};

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsResponse {
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

pub const ERROR_STYLE:Style = Style::new()
.red();

pub const ERROR_STYLE_BOLD:Style = Style::new()
.red()
.bold();

pub const WARN_STYLE:Style = Style::new()
.yellow();

pub const WARN_STYLE_BOLD:Style = Style::new()
.yellow()
.bold();

pub const INFO_STYLE:Style = Style::new()
.green();

pub const INFO_STYLE_BOLD:Style = Style::new()
.green()
.bold();

pub const LINE_BREAK:Style = Style::new()
.black()
.bold();

pub static SUCCESS_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn increment_success_counter() {
    SUCCESS_COUNTER.fetch_add(1, Ordering::Relaxed);
}

pub fn get_success_counter() -> usize {
    SUCCESS_COUNTER.load(Ordering::Relaxed)
}