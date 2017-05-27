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

type Tuple = (u32, u32);
struct BlurredEdge {
    pub orig_pos: Tuple,
    pub dims: Tuple,

    pub paste_pos: Tuple,
    pub rotated_offset: Tuple,
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

    let edges = [// horizontal
                 BlurredEdge {
                     orig_pos: (0, 0),
                     dims: (dims.0, MARGIN + BLUR_MARGIN),
                     paste_pos: (0, 0),
                     rotated_offset: (0, dims_orig.1 + BLUR_MARGIN),
                 },

                 // vertical
                 BlurredEdge {
                     orig_pos: (0, MARGIN + BLUR_MARGIN),
                     dims: (MARGIN + BLUR_MARGIN, dims.1 - MARGIN * 2 - BLUR_MARGIN * 2),
                     paste_pos: (0, MARGIN + BLUR_MARGIN),
                     rotated_offset: (dims_orig.0 + BLUR_MARGIN, 0),
                 }];

    // apply blurred edges
    for edge in &edges {
        let view = resized
            .sub_image(edge.orig_pos.0, edge.orig_pos.1, edge.dims.0, edge.dims.1)
            .to_image();
        let view = image::imageops::blur(&view, 7.0);
        resized.copy_from(&view, edge.paste_pos.0, edge.paste_pos.1);

        let rot = image::imageops::rotate180(&view);
        resized.copy_from(&rot,
                          edge.paste_pos.0 + edge.rotated_offset.0,
                          edge.paste_pos.1 + edge.rotated_offset.1);
    }

    // copy original image across
    resized.copy_from(&image, MARGIN, MARGIN);

    Ok(resized)
}
