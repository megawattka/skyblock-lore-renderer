use std::io::Cursor;

use image::{ImageFormat, Rgb};

pub fn to_png_bytes(img: &image::RgbImage) -> anyhow::Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());
    img.write_to(&mut cursor, ImageFormat::Png)?;
    Ok(cursor.into_inner())
}

pub fn hex_to_rgb(hex: &str) -> Option<Rgb<u8>> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let packed = u32::from_str_radix(hex, 16).ok()?;
    
    Some(Rgb([
        ((packed >> 16) & 0xFF) as u8,
        ((packed >> 8) & 0xFF) as u8,
        (packed & 0xFF) as u8,
    ]))
}

pub const fn color_from_code(code: &char) -> Option<Rgb<u8>> {
    match code {
        '0' => Some(Rgb([0, 0, 0])),
        '1' => Some(Rgb([0, 0, 170])),
        '2' => Some(Rgb([0, 170, 0])),
        '3' => Some(Rgb([0, 170, 170])),
        '4' => Some(Rgb([170, 0, 0])),
        '5' => Some(Rgb([170, 0, 170])),
        '6' => Some(Rgb([255, 170, 0])),
        '7' => Some(Rgb([170, 170, 170])),
        '8' => Some(Rgb([85, 85, 85])),
        '9' => Some(Rgb([85, 85, 255])),
        'a' => Some(Rgb([85, 255, 85])),
        'b' => Some(Rgb([85, 255, 255])),
        'c' => Some(Rgb([255, 85, 85])),
        'd' => Some(Rgb([255, 85, 255])),
        'e' => Some(Rgb([255, 255, 85])),
        'f' => Some(Rgb([255, 255, 255])),
        _ => None
    }
}