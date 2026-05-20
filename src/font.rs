use ab_glyph::{Font, FontRef, PxScale, ScaleFont};

use crate::config::FONT_SIZE;

/// Load the embedded Faithful font
pub fn load_font() -> anyhow::Result<FontRef<'static>> {
    let font_data = include_bytes!("../fonts/faithful-unicode.ttf");
    FontRef::try_from_slice(font_data).map_err(Into::into)
}

/// Calculate the pixel width of text at the given scale
pub fn calculate_text_width(font: &impl Font, scale: f32, text: &str) -> f32 {
    let scaled = font.as_scaled(scale);
    text.chars()
        .map(|c| {
            let glyph = scaled.scaled_glyph(c);
            scaled.h_advance(glyph.id)
        })
        .sum()
}

/// Get the default scale for rendering
pub fn default_scale() -> PxScale {
    PxScale::from(FONT_SIZE)
}