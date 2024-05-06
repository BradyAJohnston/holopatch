use image::{GenericImageView, ImageBuffer, Rgba};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn main() {
    let dir_path = get_directory_path();
    let mut pngs = get_pngs(&dir_path);
    sort_pngs(&mut pngs);

    let (width, height) = get_image_dimensions(&pngs[0]);

    for (i, window) in pngs.windows(48).enumerate() {
        let imgbuf = Arc::new(Mutex::new(create_image_buffer(width, height)));
        process_images(window, &imgbuf, width, height);
        save_image(&imgbuf, &format!("image{}", i));
    }
}

fn get_directory_path() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide the directory path as an argument");
    }
    Path::new(&args[1]).to_path_buf()
}

fn get_pngs(dir_path: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir_path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
        .map(|e| e.path())
        .collect()
}

fn sort_pngs(pngs: &mut Vec<PathBuf>) {
    pngs.sort_by_key(|path| path.file_name().unwrap().to_owned());
}

fn get_image_dimensions(path: &Path) -> (u32, u32) {
    match image::open(path) {
        Ok(img) => img.dimensions(),
        Err(_) => panic!("Error opening image"),
    }
}

fn create_image_buffer(width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let grid_width = 8;
    let grid_height = 6;
    ImageBuffer::new(width * grid_width, height * grid_height)
}

fn process_images(pngs: &[PathBuf], imgbuf: &Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>, width: u32, height: u32) {
    let grid_width = 8;
    let grid_height = 6;

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
}

fn save_image(imgbuf: &Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>, prefix: &str) {
    let file_name = format!("{}_stitched.png", prefix);
    let path = Path::new(&file_name);
    imgbuf.lock().unwrap().save(path).unwrap();
}