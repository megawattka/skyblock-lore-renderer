use image::Rgb;
use regex::Regex;

use crate::{
    config::{
        COLOR_BLUE,
        COLOR_GOLD,
        COLOR_PURPLE,
        T4_ENCHANTMENTS,
        T3_ENCHANTMENTS
    },
    core::parser::parse_roman_numeral,
    models::RenderOptions
};

/// Determines the color for an enchantment based on its tier
pub fn get_enchantment_color(text: &str, regex: &Regex) -> Option<Rgb<u8>> {
    let captures = regex.captures(text)?;
    let enchantment = captures.get(1)?.as_str();

    let tier = parse_roman_numeral(captures.get(2)?.as_str())?;

    // Enchants that have max t3 do not have t4
    if T3_ENCHANTMENTS.contains(&enchantment) && tier == 3 {
        return Some(COLOR_GOLD)
    }

    let is_contains = T4_ENCHANTMENTS.contains(&enchantment);
    let lowest_tier = if is_contains { 3 } else { 5 };

    let key = tier - lowest_tier;

    match key {
        ..=0 => Some(COLOR_BLUE),  // Normal Sharpness V
        1 => Some(COLOR_GOLD),  // Sharpness VI
        2.. => Some(COLOR_PURPLE),  // Sharpness VII
    }
}

/// Check if text should use enchantment coloring (not gray, not bold)
pub fn should_use_enchantment_color(format_codes: &[char], options: &RenderOptions) -> bool {
    options.recolor_enchantments && !format_codes.contains(&'7') && !format_codes.contains(&'l')
}