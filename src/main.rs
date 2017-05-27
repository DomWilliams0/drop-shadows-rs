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
    const MARGIN: u32 = 20;
    const BLUR_MARGIN: u32 = 10;

    // create resized image
    let dims_orig = image.dimensions();
    let mut resized = image::DynamicImage::new_rgba8(dims_orig.0 + MARGIN * 2,
                                                     dims_orig.1 + MARGIN * 2);
    let dims = resized.dimensions();

    // create shadow
    let black = image::Rgba::<u8> { data: [0, 0, 0, 255] };
    for y in MARGIN + 1..dims.1 - MARGIN {
        for x in MARGIN + 1..dims.0 - MARGIN {
            resized.put_pixel(x, y, black);
        }
    }

    // blur edges
    // TODO ooer
    let edges =
        [((0, 0, dims.0, MARGIN + BLUR_MARGIN), (0, 0), (0, dims_orig.1 + BLUR_MARGIN)),
         ((0, MARGIN + BLUR_MARGIN, MARGIN + BLUR_MARGIN, dims.1 - MARGIN * 2 - BLUR_MARGIN * 2),
          (0, MARGIN + BLUR_MARGIN),
          (dims_orig.0 + BLUR_MARGIN, 0))];

    // apply blurred edges
    for tup in &edges {
        let ((x, y, w, h), (paste_x, paste_y), (rot_off_x, rot_off_y)) = *tup;
        let view = resized.sub_image(x, y, w, h).to_image();
        let view = image::imageops::blur(&view, 7.0);
        resized.copy_from(&view, paste_x, paste_y);

        let rot = image::imageops::rotate180(&view);
        resized.copy_from(&rot, paste_x + rot_off_x, paste_y + rot_off_y);
    }

    // copy original image across
    resized.copy_from(&image, MARGIN, MARGIN);

    Ok(resized)
}
