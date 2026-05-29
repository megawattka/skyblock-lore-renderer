use image::Rgb;

/// Font size in pixels
pub const FONT_SIZE: f32 = 32.0;
/// Line height in pixels
pub const LINE_HEIGHT: i32 = 32;
/// Horizontal padding
pub const PADDING_X: i32 = 16;
/// Vertical padding
pub const PADDING_Y: i32 = 16;
/// Extra vertical space at bottom
pub const BOTTOM_MARGIN: u32 = 32;

pub static T4_ENCHANTMENTS: &[&str] = &[
    "Experience",
    "Life Steal",
    "Scavenger",
    "Looting",
];

pub static T3_ENCHANTMENTS: &[&str] = &[
    "Mana Steal",
    "Fire Aspect"
];

pub const DEFAULT_COLOR: Rgb<u8> = Rgb([255, 255, 255]);
pub const BACKGROUND_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

pub const COLOR_GOLD: Rgb<u8> = Rgb([255, 170, 0]);
pub const COLOR_BLUE: Rgb<u8> = Rgb([85, 85, 255]);
pub const COLOR_PURPLE: Rgb<u8> = Rgb([170, 0, 170]);