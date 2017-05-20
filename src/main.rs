extern crate image;

use std::process;
use std::fs;
use image::GenericImage;

fn main() {

    match run() {
        Err(err) => {
            println!("Error: {:?}", err);
            process::exit(1);
        }

        Ok(_) => process::exit(0),
    }
}

fn run() -> Result<(), image::ImageError> {
    let in_path = "/tmp/image.png";
    let out_path = "/tmp/image-pretty.png";

    let image = image::open(in_path)?;
    println!("Loaded image");

    println!("Messing with it...");
    let image = mess_with_image(image)?;

    // save
    println!("Saving to '{}'", out_path);
    let mut out_file = fs::File::create(out_path)?;
    image.save(&mut out_file, image::ImageFormat::PNG)?;

    Ok(())

}

fn mess_with_image(image: image::DynamicImage) -> Result<image::DynamicImage, image::ImageError> {
    const MARGIN: u32 = 40;

    let dims = image.dimensions();
    let mut resized = image::DynamicImage::new_rgba8(dims.0 + MARGIN * 2, dims.1 + MARGIN * 2);
    resized.copy_from(&image, MARGIN, MARGIN);

    Ok(resized)
}
