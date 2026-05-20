use ab_glyph::FontRef;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;

use crate::config::{
    BACKGROUND_COLOR,
    BOTTOM_MARGIN,
    COLOR_FORMATS,
    DEFAULT_COLOR,
    LINE_HEIGHT,
    PADDING_X,
    PADDING_Y,
};
use crate::enchantments::{get_enchantment_color, should_use_enchantment_color};
use crate::font::{calculate_text_width, default_scale};
use crate::parser::TextSegment;
use regex::Regex;

/// Render parsed lore lines into an RGB image
pub fn render_lore(
    segments: &[Vec<TextSegment>],
    font: &FontRef,
    enchant_regex: &Regex,
) -> RgbImage {
    let (width, height) = calculate_dimensions(segments, font);
    let mut img = ImageBuffer::from_pixel(width, height, BACKGROUND_COLOR);

    let scale = default_scale();
    let mut y = PADDING_Y;

    for line in segments {
        let mut x = PADDING_X;
        for segment in line {
            let color = resolve_color(segment, enchant_regex);
            let is_bold = segment.format_codes.contains(&"§l".to_string());

            draw_text_segment(&mut img, color, x, y, scale, font, &segment.text, is_bold);

            let width = calculate_text_width(font, scale.x, &segment.text);
            x += width as i32;
        }
        y += LINE_HEIGHT;
    }

    img
}

/// Calculate required image dimensions
fn calculate_dimensions(segments: &[Vec<TextSegment>], font: &FontRef) -> (u32, u32) {
    let height = (segments.len() as u32 * LINE_HEIGHT as u32) + BOTTOM_MARGIN;
    let max_width = segments
        .iter()
        .map(|line| {
            let text: String = line.iter().map(|s| &*s.text).collect();
            calculate_text_width(font, default_scale().x, &text).round() as u32
        })
        .max()
        .unwrap_or(0);

    (max_width + (PADDING_X as u32 * 2), height)
}

/// Resolve the color for a text segment
fn resolve_color(segment: &TextSegment, enchant_regex: &Regex) -> Rgb<u8> {
    if enchant_regex.is_match(&segment.text) && should_use_enchantment_color(&segment.format_codes)
    {
        if let Some(color) = get_enchantment_color(&segment.text, enchant_regex) {
            return color;
        }
    }

    segment
        .format_codes
        .iter()
        .fold(DEFAULT_COLOR, |acc, code| {
            COLOR_FORMATS.get(code.as_str()).copied().unwrap_or(acc)
        })
}

/// Draw a text segment with optional bold effect (offset shadow)
fn draw_text_segment(
    img: &mut RgbImage,
    color: Rgb<u8>,
    x: i32,
    y: i32,
    scale: ab_glyph::PxScale,
    font: &FontRef,
    text: &str,
    is_bold: bool,
) {
    if is_bold {
        // Draw offset shadow for bold effect
        draw_text_mut(img, color, x + 2, y, scale, font, text);
    }
    draw_text_mut(img, color, x, y, scale, font, text);
}