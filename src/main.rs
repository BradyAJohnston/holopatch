use image::{GenericImageView, ImageBuffer};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn main() {
    // Get the directory path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide the directory path as an argument");
    }
    let dir_path = Path::new(&args[1]);

    // List of PNGs
    let mut pngs: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
        .map(|e| e.path())
        .collect();

    // Sort the PNGs by their names
    pngs.sort_by_key(|path| path.file_name().unwrap().to_owned());

    // Determine grid size
    let grid_width = 8;
    let grid_height = 6;

    // Open the first image to get the dimensions
    let (width, height) = match image::open(&pngs[0]) {
        Ok(img) => img.dimensions(),
        Err(_) => panic!("Error opening image"),
    };

    // Create a new image buffer
    let imgbuf = Arc::new(Mutex::new(ImageBuffer::new(width * grid_width, height * grid_height)));

    // Iterate over the images and copy them into the image buffer
    pngs.par_iter().take(48).enumerate().for_each(|(i, png)| {
        let img = match image::open(png) {
            Ok(img) => img,
            Err(_) => panic!("Error opening image"),
        };
    
        let x = (grid_width - 1 - (i as u32 % grid_width)) * width;
        let y = (grid_height - 1 - (i as u32 / grid_width)) * height;
    
        for (px, py, pixel) in img.to_rgba8().enumerate_pixels() {
            let mut imgbuf = imgbuf.lock().unwrap();
            imgbuf.put_pixel(x + px, y + py, *pixel);
        }
    });

    // Save the stitched image
    imgbuf.lock().unwrap().save("stitched.png").unwrap();
}