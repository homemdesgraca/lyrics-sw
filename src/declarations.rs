use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsResponse {
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}