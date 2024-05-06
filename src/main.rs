use image::{GenericImageView, ImageBuffer, Rgba};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the directory path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide the directory path as an argument");
    }
    let dir_path = Path::new(&args[1]);

    // List of PNGs
    let pngs: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
        .map(|e| e.path())
        .collect();

    // Determine grid size
    // let grid_size = (pngs.len() as f64).sqrt().ceil() as u32;
    let grid_width = 8;
    let grid_height = 6;

    // Open the first image to get the dimensions
    let (width, height) = match image::open(&pngs[0]) {
        Ok(img) => img.dimensions(),
        Err(_) => panic!("Error opening image"),
    };

    // Create a new image buffer
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(width * grid_width, height * grid_height);

    // Iterate over the images and copy them into the image buffer
    for (i, png) in pngs.iter().take(48).enumerate() {
        let img = match image::open(png) {
            Ok(img) => img,
            Err(_) => panic!("Error opening image"),
        };

        let x = (i as u32 % grid_width) * width;
        let y = (i as u32 / grid_width) * height;

        for (px, py, pixel) in img.to_rgba8().enumerate_pixels() {
            imgbuf.put_pixel(x + px, y + py, *pixel);
        }
    }

    // Save the stitched image
    imgbuf.save("stitched.png").unwrap();
}
