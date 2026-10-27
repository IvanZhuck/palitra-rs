use anyhow::Result;
use clap::{Parser, ValueEnum};
use comfy_table::presets::UTF8_FULL;
use palitra_rs::colors::{build_colors_palette, extract_colors_from_image, save_palette_as_file, ColorFormat};
use palitra_rs::validation::validate_parameter_colors;
use comfy_table::Table;

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Table,
    List,
    JsonObject,
    JsonArray,
    CssVariables,
}

/// Generate a color palette from an image
#[derive(Parser, Debug)]
struct Args {
    /// Number of colors to generate
    #[arg(short = 'c', long, default_value_t = 16, value_parser = validate_parameter_colors)]
    colors: u8,

    /// Color format
    #[arg(short = 'f', long, visible_alias = "cf", default_value_t=ColorFormat::Hex, value_enum)]
    color_format: ColorFormat,

    /// Output format
    #[arg(short = 'o', long, visible_alias = "of", default_value_t=OutputFormat::Table, value_enum)]
    output_format: OutputFormat,

    /// Path to the output image files
    #[arg(short = 'i', long, visible_alias = "oif")]
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

    match args.output_format {
        OutputFormat::Table => {
            let mut total_weight = 0;
            let mut table = Table::new();
            table.load_style(UTF8_FULL.with_rounded_corners())
                .set_header(vec!["Hex", "RGB", "HSL", "CIELab", "Weight"]);

            for color in &palette {
                total_weight += color.weight;
            }

            for color in &palette {
                table.add_row(vec![
                    color.get_formatted_code(&ColorFormat::Hex),
                    color.get_formatted_code(&ColorFormat::RGB),
                    color.get_formatted_code(&ColorFormat::HSL),
                    color.get_formatted_code(&ColorFormat::Lab),
                    format!("{:.2}%", color.weight as f32 / (total_weight as f32 * 0.01))
                ]);
            }

            println!("{table}");
        }
        OutputFormat::List => {
            for color in palette {
                println!("{}", color.get_formatted_code(&args.color_format))
            }
        }
        OutputFormat::JsonArray => {
            let items: Vec<String> = palette
                .iter()
                .map(|color| format!("  \"{}\"", color.get_formatted_code(&args.color_format)))
                .collect();

            println!("[\n{}\n]", items.join(",\n"));
        }
        OutputFormat::JsonObject => {
            let items: Vec<String> = palette
                .iter()
                .enumerate()
                .map(|(i, color)| format!("  \"color_{}\": \"{}\"", i, color.get_formatted_code(&args.color_format)))
                .collect();

            println!("{{\n{}\n}}", items.join(",\n"));
        }
        OutputFormat::CssVariables => {},
    }

    Ok(())
}
