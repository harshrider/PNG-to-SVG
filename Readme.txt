PNG to SVG Converter

This project converts PNG images into SVG format using a combination of grayscale conversion and Sobel edge detection, implemented in Rust. The Sobel edge detection identifies the edges in the images and the resulting edges are then converted into vector format.

Features

- Convert PNG images to grayscale.
- Apply Sobel edge detection on grayscale images.
- Generate SVG files from processed images.

How to run
cargo run
should be 41 packages in total
- Use option 1 to see the steps it takes to create an SVG image
- Use option 2 to chose your own image, make sure it is a .png and there are no capitalization errors, can use the ones provided
- Use option 3 or 'q' to exit the program

Dependencies

The project uses several external crates:

- `image` for image processing.
- `rayon` for parallel processing.
- `svg` for generating SVG files.

To install these, add the following lines to your `Cargo.toml`:

```toml
[dependencies]
image = "0.23"
rayon = "1.5"
svg = "0.8"


Extra Info:
Faster by about 15 seconds with Parrell implmentation of the code with large images (4k),
Turns out the sobel opperator and rust itself is a verry fast language, 
Although with biggerfiles, the grea scale alpha function can take time doing each pixel
individualy 
Most of this code was run and tested on WSL and vim localy, and did not constantly commit during the process. 
