use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use regex::Regex;
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};

pub(crate) fn parse_lore_lines(lines: &[&str]) -> Vec<Vec<(String, String)>> {
    let pattern = Regex::new(r"((?:§[0-9a-fklmnor])+)([^§]*)").unwrap();
    
    lines.iter()
        .map(|line| {
            pattern.captures_iter(line)
                .map(|caps| (caps[1].to_string(), caps[2].to_string()))
                .collect()
        })
        .collect()
}

pub(crate) fn calculate_text_width(font: &impl Font, scale: f32, text: &str) -> f32 {
    let scaled = font.as_scaled(scale);
    
    text.chars()
        .map(|c| {
            let glyph = scaled.scaled_glyph(c);
            scaled.h_advance(glyph.id)
        })
        .sum()
}

pub(crate) fn split_format_codes(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    chars.chunks_exact(2)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

pub(crate) fn draw_text(img: &mut RgbImage, color: Rgb<u8>, x: i32, y: i32, scale: PxScale, font: &FontRef, text: &str, is_bold: bool) {
    if is_bold {
       draw_text_mut(img, color, x + 2, y, scale, font, text); // offset by 2
    }
    draw_text_mut(img, color, x, y, scale, font, text);
}