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

## How to build from source code
## How to use
## How fast is it
## FAQ



