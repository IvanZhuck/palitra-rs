use anyhow::Result;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgba};
use std::collections::HashMap;

const RESIZED_IMG_MAX_WIDTH: u32 = 200;
const RESIZED_IMG_MAX_HEIGHT: u32 = 200;
const EXPECTED_SAMPLES_COUNT: u32 = 8000;

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub weight: u32,
}

struct ColorSum {
    r: u32,
    g: u32,
    b: u32,
    count: u32,
}

/// Extracts colors from an image file
pub fn extract_colors_from_image(img_file_path: &String) -> Result<Vec<Color>> {
    let image = image::open(img_file_path)?;
    let resized_image = image.resize(
        RESIZED_IMG_MAX_WIDTH,
        RESIZED_IMG_MAX_HEIGHT,
        FilterType::Nearest,
    );

    let color_sums: HashMap<u16, ColorSum> = collect_color_sums(&resized_image);
    Ok(aggregate_colors(&color_sums))
}

/// Samples pixels from the image and aggregates them into quantized color sums
fn collect_color_sums(image: &DynamicImage) -> HashMap<u16, ColorSum> {
    let mut color_sums: HashMap<u16, ColorSum> = HashMap::new();

    let width = image.width();
    let height = image.height();

    let sample_step_px = 1.max(
        ((width * height / EXPECTED_SAMPLES_COUNT) as f64)
            .sqrt()
            .ceil() as usize,
    );

    for y in (0..height).step_by(sample_step_px) {
        for x in (0..width).step_by(sample_step_px) {
            let pixel = image.get_pixel(x, y);

            // Ignore transparent pixels
            if pixel[3] == 0 {
                continue;
            }

            let index = index_from_pixel(&pixel);

            color_sums
                .entry(index)
                .and_modify(|color_sum| {
                    color_sum.r += pixel[0] as u32;
                    color_sum.g += pixel[1] as u32;
                    color_sum.b += pixel[2] as u32;
                    color_sum.count += 1;
                })
                .or_insert(ColorSum {
                    r: pixel[0] as u32,
                    g: pixel[1] as u32,
                    b: pixel[2] as u32,
                    count: 1,
                });
        }
    }

    color_sums
}

/// Creates a flat 15-bit index for the given pixel
fn index_from_pixel(pixel: &Rgba<u8>) -> u16 {
    // Drop the lowest 3 bits of each color channel
    let simplified_r = (pixel[0] >> 3) as u16;
    let simplified_g = (pixel[1] >> 3) as u16;
    let simplified_b = (pixel[2] >> 3) as u16;

    (simplified_r << 10) | (simplified_g << 5) | simplified_b
}

/// Computes average colors from accumulated color sums and sorts them by weight
fn aggregate_colors(color_sums: &HashMap<u16, ColorSum>) -> Vec<Color> {
    let mut colors: Vec<Color> = vec![];

    for (_, color_sum) in color_sums {
        colors.push(Color {
            r: (color_sum.r / color_sum.count) as u8,
            g: (color_sum.g / color_sum.count) as u8,
            b: (color_sum.b / color_sum.count) as u8,
            weight: color_sum.count,
        });
    }

    colors.sort_by_key(|color| color.weight);
    colors.reverse();

    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_colors_from_image() {
        let file_path = String::from("test_image.jpg");
        assert_eq!(extract_colors_from_image(&file_path).unwrap().len(), 947);
        assert_eq!(
            extract_colors_from_image(&file_path).unwrap()[0].weight,
            127
        );

        let file_path = String::from("test_image.png");
        assert!(extract_colors_from_image(&file_path).is_err());
    }

    #[test]
    fn test_index_from_pixel() {
        let pixel = Rgba::from([255, 34, 122, 100]);
        assert_eq!(index_from_pixel(&pixel), 31887);

        let pixel = Rgba::from([0, 0, 0, 0]);
        assert_eq!(index_from_pixel(&pixel), 0);

        let pixel = Rgba::from([255, 255, 255, 0]);
        assert_eq!(index_from_pixel(&pixel), 32767);
    }

    #[test]
    fn test_collect_color_sums() {
        let image = image::open("test_image.jpg").unwrap();
        let resized_image = image.resize(
            RESIZED_IMG_MAX_WIDTH,
            RESIZED_IMG_MAX_HEIGHT,
            FilterType::Nearest,
        );

        let color_sums: HashMap<u16, ColorSum> = collect_color_sums(&resized_image);
        assert_eq!(color_sums.len(), 947);
    }

    #[test]
    fn test_aggregate_colors() {
        let mut color_sums: HashMap<u16, ColorSum> = HashMap::new();

        assert_eq!(aggregate_colors(&color_sums).len(), 0);

        color_sums.insert(
            0,
            ColorSum {
                r: 0,
                g: 0,
                b: 0,
                count: 1,
            },
        );

        color_sums.insert(
            562,
            ColorSum {
                r: 255,
                g: 255,
                b: 255,
                count: 5,
            },
        );

        color_sums.insert(
            78,
            ColorSum {
                r: 100,
                g: 100,
                b: 100,
                count: 3,
            },
        );

        assert_eq!(aggregate_colors(&color_sums).len(), 3);
        assert_eq!(aggregate_colors(&color_sums)[0].weight, 5);
        assert_eq!(aggregate_colors(&color_sums)[1].weight, 3);
        assert_eq!(aggregate_colors(&color_sums)[2].r, 0);
    }
}
