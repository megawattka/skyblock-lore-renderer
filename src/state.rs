use std::sync::Arc;

use ab_glyph::FontRef;
use log::info;
use regex::Regex;

use crate::font::load_font;

#[derive(Debug, Clone)]
pub struct AppState {
    pub font: FontRef<'static>,
    pub enchant_regex: Regex,
}

pub fn build_state() -> anyhow::Result<Arc<AppState>> {
    let font = load_font()?;
    info!("Faithful font loaded successfully");

    let enchant_regex = Regex::new(r"^([A-Za-z ]+) ([IVX]+)")?;
    Ok(Arc::new(AppState { font, enchant_regex }))
}