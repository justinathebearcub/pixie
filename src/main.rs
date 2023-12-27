use clap::Parser;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    file_path: std::path::PathBuf,
    pixelate_width: u32,
    pixelate_height: u32,
}

type ImageMatrix = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn resize(img: &ImageMatrix, new_dims: (u32, u32)) -> ImageMatrix {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dims;

    let mut resized_img = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in resized_img.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width as f32 / new_width as f32)) as u32;
        let old_y = (new_y as f32 * (old_height as f32 / new_height as f32)) as u32;

        *pixel = *img.get_pixel(old_x, old_y)
    }

    resized_img
}

fn pixelate(img: &DynamicImage, new_dims: (u32, u32)) -> ImageMatrix {
    let old_dims = img.dimensions();

    let img = img.to_rgba8();

    let small_img = resize(&img, ((old_dims.0 / new_dims.0), (old_dims.1 / new_dims.1)));

    resize(&small_img, old_dims)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file_path = args.file_path;
    let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
    let pixelate_width = args.pixelate_width;
    let pixelate_height = args.pixelate_height;

    let image = ImageReader::open(&file_path)?.decode()?;
    let (width, height) = image.dimensions();

    let image_matrix = pixelate(&image, (pixelate_width, pixelate_height));
    let pixie_file_name = format!(
        "pixie_{:03}_{:03}_{}",
        pixelate_width, pixelate_height, file_name
    );
    let _ = image_matrix.save(&pixie_file_name);
    println!("{pixie_file_name}: ({width}, {height})");

    Ok(())
}
