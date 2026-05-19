mod lore;

use std::{env, path::PathBuf, time::Instant};

use dotenvy::dotenv;
use log::info;
use phf::phf_map;
use image::{ImageBuffer, Rgb, RgbImage};
use ab_glyph::{FontRef, PxScale};

use lore::{parse_lore_lines, calculate_text_width, split_format_codes, draw_text};

static COLOR_FORMATS: phf::Map<&str, Rgb<u8>> = phf_map! {
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

static DEFAULT_COLOR: Rgb<u8> = Rgb([255, 255, 255]);

fn main() -> anyhow::Result<()> {
    // Set up logging and load environment variables
    dotenv().ok();
    env_logger::init();
    println!("Skyblock Lore Renderer - Starting up...");

    // Load font with ab_glyph
    let font_data = include_bytes!("../fonts/faithful-unicode.ttf");
    let font = FontRef::try_from_slice(font_data)?;
    info!("Faithful font loaded successfully");

    // Handle command-line arguments
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        let path = PathBuf::from(&args[0]);
        eprintln!("Usage: {} <input.txt> <output.png>", path.file_name().unwrap().to_string_lossy());
        std::process::exit(1);
    }
    // Read input file and prepare for rendering
    let start = Instant::now();
    let input = std::fs::read_to_string(&args[1]).expect("Failed to open file");
    let lines: Vec<&str> = input.lines().collect();
    info!("Input file read successfully with {} lines", lines.len());

    // Calculate image dimensions based on text content
    let height = (lines.len() * 32) as u32 + 32;
    let scale = PxScale::from(32.0);

    // Parse lore lines and determine image width
    let mut initial_y = 16;
    let parsed = parse_lore_lines(&lines);
    let width = parsed.iter().map(|block| {
        let text: Vec<String> = block.iter().map(|(_, text)| text.to_string()).collect();
        calculate_text_width(&font, 32.0, &text.join("")).round() as u32
    }).max().unwrap() + 32;

    let mut img: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([0, 0, 0]));
    info!("Image created with dimensions: {}x{}", width, height);
    
    // Draw text onto the image
    for block in &parsed {
        let mut initial_x = 16;
        for (format, text) in block {
            let splitted = split_format_codes(&format);
            let color = splitted
                .iter()
                .fold(DEFAULT_COLOR, |acc, code| {
                    COLOR_FORMATS.get(code).copied().unwrap_or(acc)
                });
            let is_bold = splitted.contains(&"§l".into());
            draw_text(
                &mut img,
                color,
                initial_x,
                initial_y,
                scale,
                &font,
                &text,
                is_bold
            );
            let calculated = calculate_text_width(&font, 32.0, &text);
            initial_x += calculated as i32;
        }
        initial_y += 32;
    }

    img.save(&args[2])?;
    info!("Image saved successfully to {}", &args[2]);
    println!("Done! Total execution time: {:.2?}", start.elapsed());

    Ok(())
}