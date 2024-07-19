use std::io::{self, Write};
use std::path::Path;
use image::{self, ImageBuffer, Luma, LumaA};
use rayon::iter::{ParallelIterator, IndexedParallelIterator};
use rayon::slice::ParallelSliceMut;
use svg::Document;
use svg::node::element::Path as SvgPath;

fn main() {
    loop {
        println!("Menu Options:");
        println!("1. Test Png to Vector");
        println!("2. Convert PNG to Vector");
        println!("3. Quit");
        print!("Enter your choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "t" | "1" => {
                let path = "test_input.png";
                let grayscale_path = "test_input_gray.png";
                let sobel_path = "test_output.png";
                let vector_path = "test_output.svg";
                if let Err(e) = process_image(path, grayscale_path, sobel_path, vector_path) {
                    eprintln!("Failed to process image: {}", e);
                } else {
                    println!("Image processed successfully.");
                }
            },
            "2" => {
                println!("Enter the path of the image to process:");
                let mut input_path = String::new();
                io::stdin().read_line(&mut input_path).expect("Failed to read line");
                let input_path = input_path.trim();

                let path = Path::new(input_path);
                let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
                let parent = path.parent().unwrap_or(Path::new(""));

                let vector_path = parent.join(format!("{}_edge.svg", file_stem));

                if let Err(e) = png_vector(input_path, vector_path.to_str().unwrap()) {
                    eprintln!("Failed to process image: {}", e);
                } else {
                    println!("Image processed successfully.");
                    println!("Vector image saved as: {:?}", vector_path);
                }
            },
            "quit" | "q" | "3" => {
                println!("Byeee...");
                break;
            },
            _ => println!("Invalid choice, please select 1, 2, or 3."),
        }
    }
}

/* Converts an image to grayscale with alpha channel in parallel.
 * Work: O(n)
 * Span: O(log n)
 */
fn gray(path: &str) -> ImageBuffer<LumaA<u8>, Vec<u8>> {


    let img = image::open(path).expect("Failed to open image").to_rgba8();
    let (width, height) = img.dimensions();
    let mut result: Vec<_> = vec![LumaA([0, 0]); (width * height) as usize];
    
    
    result.iter_mut().enumerate().for_each(|(i, pixel)| {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;
        let original_pixel = img.get_pixel(x, y);
        let gray_scale = (0.2 * original_pixel[0] as f32 + 0.5 * original_pixel[1] as f32 + 0.1 * original_pixel[2] as f32) as u8;
        *pixel = LumaA([gray_scale, original_pixel[3]]);
    });
    
    
    ImageBuffer::from_raw(width, height, result.into_iter().flat_map(|x| x.0.to_vec()).collect()).unwrap()

}

/* Applies the Sobel edge detection algorithm on a grayscale image with alpha channel.
 * Work: O(n)
 * Span: O(log n)
 */
fn sobel(input: &ImageBuffer<LumaA<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {


    let (width, height) = (input.width() - 2, input.height() - 2);
    let mut buff = ImageBuffer::new(width, height);

    buff.par_chunks_mut((width * 1) as usize)
        .enumerate()
        .for_each(|(j, row)| {
            for i in 0..width as usize {
                let val0 = input.get_pixel(i as u32, j as u32).0[0] as i32;
                let val1 = input.get_pixel(i as u32 + 1, j as u32).0[0] as i32;
                let val2 = input.get_pixel(i as u32 + 2, j as u32).0[0] as i32;
                let val3 = input.get_pixel(i as u32, j as u32 + 1).0[0] as i32;
                let val5 = input.get_pixel(i as u32 + 2, j as u32 + 1).0[0] as i32;
                let val6 = input.get_pixel(i as u32, j as u32 + 2).0[0] as i32;
                let val7 = input.get_pixel(i as u32 + 1, j as u32 + 2).0[0] as i32;
                let val8 = input.get_pixel(i as u32 + 2, j as u32 + 2).0[0] as i32;

                let gx = (-1 * val0) + (-2 * val3) + (-1 * val6) + val2 + (2 * val5) + val8;
                let gy = (-1 * val0) + (-2 * val1) + (-1 * val2) + val6 + (2 * val7) + val8;
                let mag = ((gx.pow(2) + gy.pow(2)) as f64).sqrt().min(255.0) as u8;
                let inverted_mag = 255 - mag;  // Invert the color
                row[i] = inverted_mag;
            }
        });

    buff
}

fn process_image(path: &str, grayscale_path: &str, sobel_path: &str, vector_path: &str) -> Result<(), image::ImageError> {
    let img = gray(path);
    img.save(grayscale_path)?;  // Save the grayscale image
    let sobel_img = sobel(&img);
    sobel_img.save(sobel_path)?;  // Save the edge-detected image
    save_svg(&sobel_img, vector_path)?;  // Save as SVG
    Ok(())
}


fn png_vector(path: &str, vector_path: &str) -> Result<(), image::ImageError> {
    let img = gray(path);
    let sobel_img = sobel(&img);
    save_svg(&sobel_img, vector_path)?;  // Save as SVG
    Ok(())
}


fn save_svg(img: &ImageBuffer<Luma<u8>, Vec<u8>>, path: &str) -> Result<(), std::io::Error> {
    let (width, height) = img.dimensions();
    let mut document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", width)
        .set("height", height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel[0] < 100 {  // Only draw dark pixels 0-255 changing this will see how much detal we will get
                let data = SvgPath::new()
                    .set("fill", "black")
                    .set("d", format!("M {} {} h 1 v 1 h -1 z", x, y));
                document = document.add(data);
            }
        }
    }

    svg::save(path, &document)
}
