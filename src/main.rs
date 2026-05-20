use std::{env, path::PathBuf, time::Instant};

use dotenvy::dotenv;
use log::info;
use regex::Regex;

use skyblock_lore_renderer::{
    font::load_font,
    parser::parse_lore_lines,
    renderer::render_lore,
};

fn main() -> anyhow::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    println!("Skyblock Lore Renderer - Starting up...");

    // Load font
    let font = load_font()?;
    info!("Faithful font loaded successfully");

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        let path = PathBuf::from(&args[0]);
        eprintln!(
            "Usage: {} <input.txt> <output.png>",
            path.file_name().unwrap().to_string_lossy()
        );
        std::process::exit(1);
    }

    // Read input
    let start = Instant::now();
    let input = std::fs::read_to_string(&args[1]).expect("Failed to open file");
    let lines: Vec<&str> = input.lines().collect();
    info!("Input file read successfully with {} lines", lines.len());

    // Parse and render
    let parsed = parse_lore_lines(&lines);
    let enchant_regex = Regex::new(r"^([A-Za-z ]+) ([IVX]+)")?;
    let img = render_lore(&parsed, &font, &enchant_regex);

    // Save output
    img.save(&args[2])?;
    info!("Image saved successfully to {}", &args[2]);
    println!("Done! Total execution time: {:.2?}", start.elapsed());

    Ok(())
}