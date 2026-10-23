use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, ValueEnum};
use palitra_rs::image::extract_colors_from_image;
use palitra_rs::validation::validate_parameter_colors;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Hex,
    Rgb,
    Json,
    CssVariables,
}

/// Generate a color palette from an image
#[derive(Parser, Debug)]
struct Args {
    /// Number of colors to generate
    #[arg(short, long, default_value_t = 6, value_parser = validate_parameter_colors)]
    colors: u8,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = Format::Hex)]
    format: Format,

    /// Path to the image file
    input_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let colors = extract_colors_from_image(&args.input_file)?;

    let colors_len = colors.len();

    for color in colors {
        println!("{:?}", color);
    }

    println!("{:?}", colors_len);
    Ok(())
}
