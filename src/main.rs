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
    let conf = Config {
        margin: 20,
        blur_margin: 20,
        blur_amount: 7.0,
        path_in: String::from("/tmp/image.png"),
        path_out: String::from("/tmp/image-pretty.png"),
    };

    let image = image::open(&conf.path_in)?;
    println!("Loaded image");


    println!("Messing with it...");
    let image = mess_with_image(image, &conf)?;

    // save
    println!("Saving to '{}'", conf.path_out);
    let mut out_file = fs::File::create(conf.path_out)?;
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

struct Config {
    margin: u32,
    blur_margin: u32,

    blur_amount: f32,

    path_in: String,
    path_out: String,
}


fn mess_with_image(image: image::DynamicImage,
                   config: &Config)
                   -> Result<image::DynamicImage, image::ImageError> {
    // create resized image
    let dims_orig = image.dimensions();
    let mut resized = image::DynamicImage::new_rgba8(dims_orig.0 + config.margin * 2,
                                                     dims_orig.1 + config.margin * 2);
    let dims = resized.dimensions();

    // create shadow
    let black = image::Rgba::<u8> { data: [0, 0, 0, 255] };
    for y in config.margin + 1..dims.1 - config.margin {
        for x in config.margin + 1..dims.0 - config.margin {
            resized.put_pixel(x, y, black);
        }
    }

    let edges = [// horizontal
                 BlurredEdge {
                     orig_pos: (0, 0),
                     dims: (dims.0, config.margin + config.blur_margin),
                     paste_pos: (0, 0),
                     rotated_offset: (0, config.margin + dims_orig.1 - config.blur_margin),
                 },

                 // vertical
                 BlurredEdge {
                     orig_pos: (0, config.margin + config.blur_margin),
                     dims: (config.margin + config.blur_margin,
                            dims.1 - config.margin * 2 - config.blur_margin * 2),
                     paste_pos: (0, config.margin + config.blur_margin),
                     rotated_offset: (config.margin + dims_orig.0 - config.blur_margin, 0),
                 }];

    // apply blurred edges
    for edge in &edges {
        let view = resized
            .sub_image(edge.orig_pos.0, edge.orig_pos.1, edge.dims.0, edge.dims.1)
            .to_image();
        let view = image::imageops::blur(&view, config.blur_amount);
        assert!(resized.copy_from(&view, edge.paste_pos.0, edge.paste_pos.1));

        let rot = image::imageops::rotate180(&view);
        assert!(resized.copy_from(&rot,
                                  edge.paste_pos.0 + edge.rotated_offset.0,
                                  edge.paste_pos.1 + edge.rotated_offset.1));
    }

    // copy original image across
    resized.copy_from(&image, config.margin, config.margin);

    Ok(resized)
}
