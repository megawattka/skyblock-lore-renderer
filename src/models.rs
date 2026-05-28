use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderOptions {
    #[serde(default = "default_bg")]
    pub background: Option<String>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self { background: default_bg() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderRequest {
    pub lore: String,

    #[serde(default)]
    pub options: Option<RenderOptions>,
}

fn default_bg() -> Option<String> { Some("#000000".into()) }

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderResponse {
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub render_time_ms: u128,
}