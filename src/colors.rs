use anyhow::Result;
use image::imageops::FilterType;
use image::{DynamicImage, Rgb, RgbImage, Rgba};
use std::cmp::PartialEq;
use std::collections::HashMap;

const RESIZED_IMG_MAX_WIDTH: u32 = 200;
const RESIZED_IMG_MAX_HEIGHT: u32 = 200;
const EXPECTED_SAMPLES_COUNT: u32 = 8000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub weight: u32,
}

impl Color {
    /// Formats the color as a HEX string (e.g. #FFFFFF).
    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Formats the color as an RGB string (e.g. rgb(255, 255, 255)).
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

#[derive(Default)]
struct ColorSum {
    r: u32,
    g: u32,
    b: u32,
    count: u32,
}

impl ColorSum {
    fn add(&mut self, pixel: &Rgba<u8>) {
        self.r += pixel[0] as u32;
        self.g += pixel[1] as u32;
        self.b += pixel[2] as u32;
        self.count += 1;
    }
}

/// Extracts colors from an image file.
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

/// Creates a color palette from a set of colors using k-means clustering.
pub fn build_colors_palette(colors: &Vec<Color>, size: u8) -> Result<Vec<Color>> {
    if colors.len() <= size as usize {
        return Ok(colors.to_vec());
    }

    let mut centers = get_color_centers(colors, size);

    for _ in 0..128 {
        let mut has_changes = false;
        let mut clusters: Vec<Vec<&Color>> = vec![Vec::new(); centers.len()];

        for color in colors {
            // Assign color to the nearest center
            let mut nearest_center_index = 0;
            let mut nearest_center_distance = u32::MAX;

            for (index, center) in centers.iter().enumerate() {
                let center_distance = two_colors_distance(color, center);
                if center_distance < nearest_center_distance {
                    nearest_center_index = index;
                    nearest_center_distance = center_distance;
                }
            }

            clusters[nearest_center_index].push(color);
        }

        for (index, center) in centers.iter_mut().enumerate() {
            // Calculate a new center for each cluster
            let cluster = &clusters[index];

            if cluster.is_empty() {
                continue;
            }

            let mut sum_r: u64 = 0;
            let mut sum_g: u64 = 0;
            let mut sum_b: u64 = 0;
            let mut sum_w: u64 = 0;

            for color in cluster {
                sum_r += color.r as u64 * color.weight as u64;
                sum_g += color.g as u64 * color.weight as u64;
                sum_b += color.b as u64 * color.weight as u64;
                sum_w += color.weight as u64;
            }

            let new_center = Color {
                r: (sum_r / sum_w) as u8,
                g: (sum_g / sum_w) as u8,
                b: (sum_b / sum_w) as u8,
                weight: sum_w.min(u32::MAX as u64) as u32,
            };

            if *center == new_center {
                continue;
            }

            has_changes = true;
            *center = new_center;
        }

        if !has_changes {
            break;
        }
    }

    centers.sort_by_key(|center| std::cmp::Reverse(center.weight));

    Ok(centers)
}

/// Saves the color palette preview as an image file.
pub fn save_palette_as_file(
    palette: &Vec<Color>,
    output_path: &String,
    square_size: u32,
) -> Result<()> {
    if palette.is_empty() {
        return Ok(());
    }

    let width = square_size * palette.len() as u32;
    let height = square_size;

    let mut img = RgbImage::new(width, height);

    for (i, color) in palette.iter().enumerate() {
        let x_offset = i as u32 * square_size;
        let pixel = Rgb([color.r, color.g, color.b]);

        for y in 0..square_size {
            for x in 0..square_size {
                img.put_pixel(x_offset + x, y, pixel);
            }
        }
    }

    img.save(output_path)?;

    Ok(())
}

/// Picks initial cluster centers using greedy weighted selection.
fn get_color_centers(colors: &Vec<Color>, size: u8) -> Vec<Color> {
    let mut centers: Vec<Color> = vec![colors[0].clone()];

    for _ in 1..size {
        let best_color = colors.iter().max_by_key(|color| {
            let mut min_distance = u32::MAX;

            for center in &centers {
                min_distance = min_distance.min(two_colors_distance(color, center));
            }

            color.weight * min_distance
        });

        if let Some(color) = best_color {
            centers.push(color.clone());
        }
    }

    centers
}

/// Calculates the squared Euclidean distance between two colors.
fn two_colors_distance(color1: &Color, color2: &Color) -> u32 {
    (color1.r as i32 - color2.r as i32).pow(2) as u32
        + (color1.g as i32 - color2.g as i32).pow(2) as u32
        + (color1.b as i32 - color2.b as i32).pow(2) as u32
}

/// Samples pixels from the image and aggregates them into quantized color sums.
fn collect_color_sums(image: &DynamicImage) -> HashMap<u16, ColorSum> {
    let mut color_sums: HashMap<u16, ColorSum> = HashMap::new();

    let width = image.width();
    let height = image.height();

    let sample_step_px = ((width * height / EXPECTED_SAMPLES_COUNT) as f64)
        .sqrt()
        .ceil()
        .max(1.0) as usize;

    let rgba = image.to_rgba8();

    for y in (0..height).step_by(sample_step_px) {
        for x in (0..width).step_by(sample_step_px) {
            let pixel = rgba.get_pixel(x, y);

            // Ignore transparent pixels
            if pixel[3] == 0 {
                continue;
            }

            let index = index_from_pixel(pixel);
            color_sums.entry(index).or_default().add(pixel);
        }
    }

    color_sums
}

/// Creates a flat 15-bit index for the given pixel.
fn index_from_pixel(pixel: &Rgba<u8>) -> u16 {
    let [r, g, b, _] = pixel.0;

    // Drop the lowest 3 bits of each color channel
    let simplified_r = (r >> 3) as u16;
    let simplified_g = (g >> 3) as u16;
    let simplified_b = (b >> 3) as u16;

    (simplified_r << 10) | (simplified_g << 5) | simplified_b
}

/// Computes average colors from accumulated color sums and sorts them by weight.
fn aggregate_colors(color_sums: &HashMap<u16, ColorSum>) -> Vec<Color> {
    let mut colors: Vec<Color> = vec![];

    for (_, color_sum) in color_sums {
        colors.push(Color {
            r: (color_sum.r / color_sum.count) as u8,
            g: (color_sum.g / color_sum.count) as u8,
            b: (color_sum.b / color_sum.count) as u8,
            weight: 1.max(color_sum.count),
        });
    }

    colors.sort_by_key(|color| std::cmp::Reverse(color.weight));

    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_formatting() {
        let color = Color {
            r: 10,
            g: 15,
            b: 255,
            weight: 1,
        };
        assert_eq!(color.to_hex_string(), "#0A0FFF");
        assert_eq!(color.to_rgb_string(), "rgb(10, 15, 255)");

        let black = Color {
            r: 0,
            g: 0,
            b: 0,
            weight: 1,
        };
        assert_eq!(black.to_hex_string(), "#000000");
        assert_eq!(black.to_rgb_string(), "rgb(0, 0, 0)");
    }

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
    fn test_build_colors_palette() {
        let colors = vec![
            Color {
                r: 255,
                g: 0,
                b: 0,
                weight: 100,
            },
            Color {
                r: 0,
                g: 255,
                b: 0,
                weight: 50,
            },
        ];

        let palette = build_colors_palette(&colors, 5).unwrap();
        assert_eq!(palette.len(), 2);

        let empty: Vec<Color> = vec![];
        assert_eq!(build_colors_palette(&empty, 5).unwrap().len(), 0);
    }

    #[test]
    fn test_get_color_centers() {
        let colors = vec![
            Color {
                r: 0,
                g: 0,
                b: 0,
                weight: 25,
            },
            Color {
                r: 255,
                g: 255,
                b: 255,
                weight: 15,
            },
            Color {
                r: 255,
                g: 0,
                b: 0,
                weight: 10,
            },
        ];

        let centers = get_color_centers(&colors, 2);
        assert_eq!(centers.len(), 2);
        assert_eq!(centers[0].r, 0);
    }

    #[test]
    fn test_two_color_distance() {
        let black = Color {
            r: 0,
            g: 0,
            b: 0,
            weight: 1,
        };
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            weight: 1,
        };
        let red = Color {
            r: 255,
            g: 0,
            b: 0,
            weight: 1,
        };
        let green = Color {
            r: 0,
            g: 255,
            b: 0,
            weight: 1,
        };
        let blue = Color {
            r: 0,
            g: 0,
            b: 255,
            weight: 1,
        };

        assert_eq!(two_colors_distance(&black, &black), 0);
        assert_eq!(two_colors_distance(&white, &white), 0);
        assert_eq!(two_colors_distance(&black, &white), 195075);

        assert_eq!(
            two_colors_distance(&black, &red),
            two_colors_distance(&red, &black)
        );
        assert_eq!(
            two_colors_distance(&white, &red),
            two_colors_distance(&red, &white)
        );

        assert_eq!(two_colors_distance(&black, &red), 65025);
        assert_eq!(two_colors_distance(&black, &green), 65025);
        assert_eq!(two_colors_distance(&black, &blue), 65025);

        let c1 = Color {
            r: 10,
            g: 30,
            b: 50,
            weight: 1,
        };
        let c2 = Color {
            r: 20,
            g: 10,
            b: 50,
            weight: 1,
        };
        assert_eq!(two_colors_distance(&c1, &c2), 500);
        assert_eq!(two_colors_distance(&c2, &c1), 500);
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
