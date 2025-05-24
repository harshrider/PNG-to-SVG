# PNG to SVG Converter

A high-performance Rust application that converts PNG images to SVG format using advanced edge detection algorithms. The converter applies grayscale transformation and Sobel edge detection to create clean vector representations of raster images.

## How It Works

1. **Grayscale Conversion**: Transforms color PNG images to grayscale
2. **Edge Detection**: Applies Sobel operator to identify image edges
3. **Vector Generation**: Converts detected edges into scalable SVG format

## Features

- ⚡ **Fast Processing**: Optimized with parallel processing using Rayon
- 🖼️ **Edge Detection**: Advanced Sobel operator implementation
- 📁 **Batch Processing**: Handle multiple images efficiently
- 🎯 **Clean Output**: Generates crisp SVG vector graphics

## Performance

- **Standard Images**: Near-instant processing
- **4K Images**: ~15 seconds faster with parallel implementation
- **Memory Efficient**: Processes large files without significant memory overhead

## Installation

### Prerequisites
- Rust 1.70+ installed
- Cargo package manager

### Dependencies
The project uses 41 total packages including:

```toml
[dependencies]
image = "0.23"    # Image processing and format handling
rayon = "1.5"     # Parallel processing for performance
svg = "0.8"       # SVG file generation
```

## Usage

### Running the Application

```bash
cargo run
```

### Interactive Menu Options

**Option 1**: Step-by-step demonstration
- Shows the complete conversion process
- Displays intermediate steps (grayscale → edge detection → SVG)

**Option 2**: Custom image conversion
- Convert your own PNG files
- **Requirements**: 
  - File must be `.png` format
  - Ensure correct capitalization in filename
  - Sample images provided for testing

**Option 3 or 'q'**: Exit program

### Example

```bash
$ cargo run
PNG to SVG Converter
1. See conversion steps
2. Convert custom image
3. Exit (q)

Enter choice: 2
Enter PNG filename: sample.png
Converting sample.png...
✓ SVG saved as sample.svg
```

## Technical Details

### Algorithm
- **Sobel Operator**: 3x3 convolution kernels for edge detection
- **Parallel Processing**: Multi-threaded pixel processing with Rayon
- **Memory Management**: Efficient handling of large image buffers

### Performance Notes
- Rust's memory safety provides excellent performance
- Sobel operator implementation is highly optimized
- Grayscale conversion handles each pixel individually (bottleneck for very large images)
- Parallel processing shows significant gains on 4K+ images

## File Structure

```
├── src/
│   └── main.rs          # Main application logic
├── samples/             # Sample PNG files for testing
├── Cargo.toml          # Project dependencies
└── README.md           # This file
```

## Development

### Testing Environment
- Primarily developed and tested on WSL (Windows Subsystem for Linux)
- Editor: Vim
- Local development workflow

### Building from Source

```bash
git clone <repository-url>
cd png-to-svg-converter
cargo build --release
```

## Troubleshooting

**Common Issues:**
- **File not found**: Ensure PNG file exists and filename is correct
- **Permission errors**: Check file read/write permissions
- **Large file processing**: Allow extra time for 4K+ images during grayscale conversion

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature-name`)
3. Commit your changes (`git commit -am 'Add feature'`)
4. Push to the branch (`git push origin feature-name`)
5. Create a Pull Request

## License

This project is open source. See LICENSE file for details.
