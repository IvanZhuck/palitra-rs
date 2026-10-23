use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, ValueEnum};

fn validate_parameter_colors(param: &str) -> Result<u8> {
    let msg = "The 'colors' parameter must be an integer between 1 and 32";
    let val: u8 = param.parse().context(msg)?;

    if !(1..=32).contains(&val) {
        bail!(msg);
    }

    Ok(val)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Hex,
    Rgb,
    Json,
    CssVariables
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

    println!("{}", args.colors);
    println!("{:?}", args.format);
    println!("{}", args.input_file);

    Ok(())
}
