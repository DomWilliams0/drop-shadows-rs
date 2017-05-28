extern crate image;

pub mod error;
pub use error::ShadowError;

pub enum ImageIo {
    Image(image::DynamicImage),
    File(String),
}

pub struct DropShadow {
    margin: u32,
    blur_margin: u32,
    blur_amount: f32,

    input: ImageIo,
    output: ImageIo,
}

impl DropShadow {
    // TODO validation and builder
    pub fn new(margin: u32,
               blur_margin: u32,
               blur_amount: f32,
               input: ImageIo,
               output: ImageIo)
               -> DropShadow {
        DropShadow {
            margin: margin,
            blur_margin: blur_margin,
            blur_amount: blur_amount,
            input: input,
            output: output,
        }
    }
}
