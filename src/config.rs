use image::Rgb;
use phf::phf_map;

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

/// Enchantments that use lower tier thresholds for coloring
pub const LOWER_TIER_ENCHANTMENTS: &[&str] = &[
    "Experience",
    "Life Steal",
    "Scavenger",
    "Looting",
];

/// Minecraft § color code mappings
pub static COLOR_FORMATS: phf::Map<&str, Rgb<u8>> = phf_map! {
    "§0" => Rgb([0, 0, 0]),
    "§1" => Rgb([0, 0, 170]),
    "§2" => Rgb([0, 170, 0]),
    "§3" => Rgb([0, 170, 170]),
    "§4" => Rgb([170, 0, 0]),
    "§5" => Rgb([170, 0, 170]),
    "§6" => Rgb([255, 170, 0]),
    "§7" => Rgb([170, 170, 170]),
    "§8" => Rgb([85, 85, 85]),
    "§9" => Rgb([85, 85, 255]),
    "§a" => Rgb([85, 255, 85]),
    "§b" => Rgb([85, 255, 255]),
    "§c" => Rgb([255, 85, 85]),
    "§d" => Rgb([255, 85, 255]),
    "§e" => Rgb([255, 255, 85]),
    "§f" => Rgb([255, 255, 255]),
};

pub const DEFAULT_COLOR: Rgb<u8> = Rgb([255, 255, 255]);
pub const BACKGROUND_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

/// Enchantment tier color overrides
pub static ENCHANTMENT_COLORS: phf::Map<i32, Rgb<u8>> = phf_map! {
    2 => Rgb([170, 0, 170]),   // Purple (rare)
    1 => Rgb([255, 170, 0]),   // Gold (epic)
};

pub const COLOR_GOLD: Rgb<u8> = Rgb([255, 170, 0]);
pub const COLOR_BLUE: Rgb<u8> = Rgb([85, 85, 255]);