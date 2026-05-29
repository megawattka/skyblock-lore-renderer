use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderOptions {
    #[serde(default = "default_bg")]
    pub background: String,

    #[serde(default = "default_recolor")]
    pub recolor_enchantments: bool
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            background: default_bg(),
            recolor_enchantments: default_recolor()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderRequest {
    pub lore: String,

    #[serde(default)]
    pub options: Option<RenderOptions>,
}

fn default_bg() -> String { "#000000".into() }
fn default_recolor() -> bool { true }

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderResponse {
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub render_time_ms: f32,
}