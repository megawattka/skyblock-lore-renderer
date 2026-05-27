use image::Rgb;
use regex::Regex;

use crate::config::{
    COLOR_BLUE,
    COLOR_GOLD,
    ENCHANTMENT_COLORS,
    LOWER_TIER_ENCHANTMENTS,
};
use crate::parser::parse_roman_numeral;

/// Determines the color for an enchantment based on its tier
pub fn get_enchantment_color(text: &str, regex: &Regex) -> Option<Rgb<u8>> {
    let captures = regex.captures(text)?;
    let enchantment = captures.get(1)?.as_str();
    let tier = parse_roman_numeral(captures.get(2)?.as_str())?;

    if enchantment == "Mana Steal" && tier == 3 {
        return Some(COLOR_GOLD)
    }

    let lowest_tier = if LOWER_TIER_ENCHANTMENTS.contains(&enchantment) {
        3
    } else {
        5
    };

    let key = tier - lowest_tier;

    if key > 2 {
        // Feather Falling X and beyond
        Some(COLOR_GOLD)
    } else if key < 1 {
        // Below minimum tier (e.g., Sharpness IV when min is V)
        Some(COLOR_BLUE)
    } else {
        ENCHANTMENT_COLORS.get(&key).copied()
    }
}

/// Check if text should use enchantment coloring (not gray, not bold)
pub fn should_use_enchantment_color(format_codes: &[String]) -> bool {
    !format_codes.contains(&"§7".to_string()) && !format_codes.contains(&"§l".to_string())
}