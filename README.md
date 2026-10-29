# palitra-rs

A command-line tool to extract dominant color palettes from images using **k-means clustering**.

Written in Rust, works on Windows, Linux, and macOS.

![Example-1](meta/images/doc-command-table-output.png)

## Features

- Extracts color palettes from images in the following formats: `JPEG`, `PNG`, `GIF`, `WEBP`, `AVIF`, `BMP`, `TIFF`, `ICO`.
- Exports colors in `HEX`, `RGB`, `HSL`, or `CIELab` formats.
- Outputs palettes as a formatted table, list, single line, or JSON.
- Supports configurable palette size (from 1 to 64 colors).
- Saves a palette preview as an image file (`JPEG`, `PNG`, `GIF`, `WEBP`, `BMP`).
- Runs as a single executable with no external dependencies.

## Examples
![Example-1: Source image](meta/images/image-1.jpg)

![Example-1: Palette of 16 colors](meta/images/image-1-palette.jpg)

By default, the program outputs a palette of 16 colors, but you can choose your own number up to 64.

```bash
./palitra-rs -c 3 -s 16 -i image-1-palette-3.jpg image-1.jpg
```
![Example-1: Palette of 3 colors](meta/images/image-1-palette-3.jpg)

```bash
./palitra-rs -c 8 -s 16 -i image-1-palette-8.jpg image-1.jpg
```
![Example-1: Palette of 8 colors](meta/images/image-1-palette-8.jpg)

```bash
./palitra-rs -c 16 -s 16 -i image-1-palette-16.jpg image-1.jpg
```
![Example-1: Palette of 16 colors](meta/images/image-1-palette-16.jpg)

```bash
./palitra-rs -c 24 -s 16 -i image-1-palette-24.jpg image-1.jpg
```
![Example-1: Palette of 24 colors](meta/images/image-1-palette-24.jpg)

```bash
./palitra-rs -c 32 -s 16 -i image-1-palette-32.jpg image-1.jpg
```
![Example-1: Palette of 32 colors](meta/images/image-1-palette-32.jpg)

Colors are sorted by frequency (weight), with the most dominant colors appearing first.

Here are a few more example images:

![Example-2](meta/images/image-8.jpg)

![Example-2: Palette of 16 colors](meta/images/image-8-palette.jpg)

![Example-3](meta/images/image-6.jpg)

![Example-3: Palette of 16 colors](meta/images/image-6-palette.jpg)

![Example-4](meta/images/image-11.jpg)

![Example-4: Palette of 16 colors](meta/images/image-11-palette.jpg)

## Building from source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then run:

```bash
git clone https://github.com/IvanZhuck/palitra-rs
cd palitra-rs
cargo build --release
```

The compiled binary will be available at `target/release/palitra-rs`.

## Usage

Ensure the file `palitra-rs` is executable and run:

```bash
./palitra-rs [OPTIONS] <INPUT_FILE>
```

Where `<INPUT_FILE>` is a path to the input image (supported formats: jpeg, png, gif, webp, avif, bmp, tiff, ico).

### Options

- `-c`, `--colors` — Number of colors to generate. Between 1 and 64, default: 16.
- `-o`, `--output-format` — Output format, possible values: table, list, line, json-object, json-array, default: table.
- `-f`, `--color-format` — Output color format, possible values: hex, rgb, hsl, lab, default: hex. Not applicable when output format is table (the table displays all formats at the same time).
- `-i`, `--output-image-file` — Path to the output image file (supported formats: jpeg, png, gif, webp, bmp).
- `-s`, `--swatch-size` — Palette swatch size in pixels for the output image. Between 10 and 512, default: 50.
- `-h`, `--help` — Print help information.

### Examples

Extract 16 dominant colors and display them in a CLI table:

```bash
./palitra-rs photo.jpg
```

Generate an 8-color palette and save a preview with 80 px swatches:

```bash
./palitra-rs photo.jpg -c 8 -i palette.png -s 80
```

Extract colors in HSL format as a JSON array:
```bash
./palitra-rs photo.jpg -c 12 -f hsl -o json-array
```

Get raw HEX values separated by spaces:
```bash
./palitra-rs photo.jpg -o line
```

## Performance

`palitra-rs` works really fast. Extracting a palette from a 4K full-color image and generating a preview takes around 50 ms. The table below shows average execution times measured with [hyperfine](https://github.com/sharkdp/hyperfine) across multiple runs on a large dataset of images:

| Resolution          | Avg File Size | Avg Execution Time |
|---------------------|--------------:|-------------------:|
| ~160px (long side)  |      16.19 KB |            4.01 ms |
| ~320px (long side)  |      32.97 KB |            4.69 ms |
| ~640px (long side)  |      87.36 KB |            6.55 ms |
| ~800px (long side)  |     124.43 KB |            7.53 ms |
| ~1024px (long side) |     185.49 KB |            8.65 ms |
| ~1280px (long side) |     268.94 KB |           10.98 ms |
| ~1920px (long side) |     531.20 KB |           16.68 ms |
| ~2048px (long side) |     591.06 KB |           17.83 ms |
| ~2560px (long side) |     859.53 KB |           23.75 ms |
| ~3840px (long side) |       1.65 MB |           44.96 ms |
| ~4096px (long side) |       1.84 MB |           51.00 ms |
| ~6000px (long side) |       4.06 MB |          102.05 ms |

## FAQ

### How does the tool handle transparent images (PNG, WebP)?
Fully transparent pixels are automatically ignored during color extraction, so transparent backgrounds won't pollute your palette with unwanted black or white colors.

### Does `palitra-rs` produce deterministic results?
Yes. Unlike standard k-means implementations with random initialization, `palitra-rs` uses a deterministic greedy selection heuristic. Running the tool multiple times on the same image will always produce the exact same palette.

### Why do the CIELab (Lab) color values slightly differ from other color pickers?

`palitra-rs` converts sRGB colors to CIELab in accordance with the official **W3C CSS Color 4** specification.

Minor discrepancies with other tools usually happen because:
- Some software (like Photoshop) uses a different color space (e.g. Adobe RGB) instead of sRGB.
- Different implementations use slightly different rounding constants for matrix transformations.

These minor variations are visually imperceptible and represent mathematically valid conversions for sRGB.

### How can I pipe the extracted colors into other tools like jq?
Use the JSON output format:
```bash
./palitra-rs image.jpg -o json-array | jq -r '.[0]'
```

### How can I process multiple JPG images from a directory with a single command?

```bash
for img in images/*.jpg; do ./palitra-rs "$img"; done
```

### How can I batch process images, generate previews, and save the results into a CSV file?

```bash
for img in images/*.jpg; do [[ "$img" == *-palette.jpg ]] && continue; p="${img%.*}-palette.jpg"; echo "$(basename "$img"),$(./palitra-rs -i "$p" --of line --cf hex "$img")"; done > output.csv
```

## License

This project is licensed under the [MIT License](LICENSE).
