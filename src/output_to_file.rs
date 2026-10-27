use crate::colors::Color;
use anyhow::{Context, Result, bail};
use image::{Rgb, RgbImage};
use std::path::Path;

/// Checks the file extension of the output path.
fn check_file_ext(output_path: &String) -> Result<()> {
    let ext = Path::new(output_path)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase());

    match ext.as_deref() {
        Some("jpeg" | "jpg" | "png" | "gif" | "webp" | "bmp") => {}
        Some(unknown) => bail!(
            "Unsupported image extension '.{}' (supported: jpeg, png, gif, webp, bmp)",
            unknown
        ),
        None => bail!("Output file '{}' has no extension", output_path),
    }

    Ok(())
}

/// Saves the color palette preview as an image file.
pub fn save_palette_as_file(
    palette: &Vec<Color>,
    output_path: &String,
    swatch_size: u32,
) -> Result<()> {
    if palette.is_empty() {
        return Ok(());
    }

    check_file_ext(output_path).with_context(|| "Output file error")?;

    let width = swatch_size * palette.len() as u32;
    let height = swatch_size;

    let mut img = RgbImage::new(width, height);

    for (i, color) in palette.iter().enumerate() {
        let x_offset = i as u32 * swatch_size;
        let pixel = Rgb([color.r, color.g, color.b]);

        for y in 0..swatch_size {
            for x in 0..swatch_size {
                img.put_pixel(x_offset + x, y, pixel);
            }
        }
    }

    img.save(output_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_check_file_ext_valid() {
        let valid_extensions = [
            "palette.jpeg",
            "palette.jpg",
            "palette.png",
            "palette.gif",
            "palette.webp",
            "palette.bmp",
            "palette.PNG",
            "palette.JPG",
            "/path/to/some/palette.webp",
        ];

        for path in valid_extensions {
            assert!(
                check_file_ext(&path.to_string()).is_ok(),
                "Expected '{}' to be valid",
                path
            );
        }
    }

    #[test]
    fn test_check_file_ext_invalid() {
        assert!(check_file_ext(&"palette.txt".to_string()).is_err());
        assert!(check_file_ext(&"palette.svg".to_string()).is_err());
        assert!(check_file_ext(&"palette.pdf".to_string()).is_err());
        assert!(check_file_ext(&"palette".to_string()).is_err());
        assert!(check_file_ext(&"/path/to/palette".to_string()).is_err());
    }

    #[test]
    fn test_save_palette_as_file_empty_palette() {
        let empty_palette: Vec<Color> = vec![];
        let file_path = "test_empty_output.png".to_string();

        let result = save_palette_as_file(&empty_palette, &file_path, 50);
        assert!(result.is_ok());

        assert!(!Path::new(&file_path).exists());
    }

    #[test]
    fn test_save_palette_as_file_invalid_extension() {
        let palette = vec![Color {
            r: 255,
            g: 0,
            b: 0,
            weight: 1,
        }];
        let file_path = "test_palette.invalid_ext".to_string();

        let result = save_palette_as_file(&palette, &file_path, 50);
        assert!(result.is_err());
        assert!(!Path::new(&file_path).exists());
    }

    #[test]
    fn test_save_palette_as_file_success() {
        let test_output_file = "test_palette_preview.png".to_string();

        let palette = vec![
            Color {
                r: 255,
                g: 0,
                b: 0,
                weight: 10,
            }, // Red
            Color {
                r: 0,
                g: 255,
                b: 0,
                weight: 20,
            }, // Green
            Color {
                r: 0,
                g: 0,
                b: 255,
                weight: 30,
            }, // Blue
        ];
        let swatch_size = 20;

        let result = save_palette_as_file(&palette, &test_output_file, swatch_size);
        assert!(result.is_ok(), "Failed to save palette: {:?}", result.err());

        assert!(Path::new(&test_output_file).exists());
        let img = image::open(&test_output_file).expect("Failed to open saved image");
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 20);
        let rgb = img.to_rgb8();
        assert_eq!(rgb.get_pixel(5, 5), &Rgb([255, 0, 0]));
        assert_eq!(rgb.get_pixel(25, 5), &Rgb([0, 255, 0]));
        assert_eq!(rgb.get_pixel(45, 5), &Rgb([0, 0, 255]));

        let _ = fs::remove_file(&test_output_file);
    }
}
