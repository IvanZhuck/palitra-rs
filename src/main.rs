use anyhow::{Result};
use clap::{Parser, ValueEnum};
use palitra_rs::colors::{build_colors_palette, extract_colors_from_image, save_palette_as_file};
use palitra_rs::validation::validate_parameter_colors;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Hex,
    Rgb,
    JsonObject,
    JsonArray,
    CssVariables,
}

/// Generate a color palette from an image
#[derive(Parser, Debug)]
struct Args {
    /// Number of colors to generate
    #[arg(short, long, default_value_t = 16, value_parser = validate_parameter_colors)]
    colors: u8,

    /// Output format
    #[arg(short, long, value_enum)]
    format: Option<Format>,

    /// Path to the output image file
    #[arg(short, long)]
    output_image_file: Option<String>,

    /// Path to the image file
    input_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let colors = extract_colors_from_image(&args.input_file)?;
    let palette = build_colors_palette(&colors, args.colors)?;

    if let Some(output_file_path) = args.output_image_file {
        save_palette_as_file(&palette, &output_file_path, 50)?;
    }

    if let Some(format) = args.format {
        match format {
            Format::Hex => {
                for color in &palette {
                    println!("{}", color.to_hex_string());
                }
            },
            Format::Rgb => {
                for color in &palette {
                    println!("{}", color.to_rgb_string());
                }
            },
            Format::JsonArray => {
                let items: Vec<String> = palette
                    .iter()
                    .map(|color| format!("  \"{}\"", color.to_hex_string()))
                    .collect();

                println!("[\n{}\n]", items.join(",\n"));
            },
            Format::JsonObject => {
                let items: Vec<String> = palette
                    .iter()
                    .enumerate()
                    .map(|(i, color)| format!("  \"color_{}\": \"{}\"", i, color.to_hex_string()))
                    .collect();

                println!("{{\n{}\n}}", items.join(",\n"));
            },
            Format::CssVariables => {},
        }
    };

    Ok(())
}
