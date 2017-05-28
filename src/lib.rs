extern crate image;

pub mod error;
pub use error::ShadowError;

use image::DynamicImage;

pub enum ImageIo {
    Image(DynamicImage),
    File(String),
}

pub struct DropShadow {
    margin: u32,
    blur_margin: u32,
    blur_amount: f32,

    input: Option<ImageIo>,
    output: Option<ImageIo>,
}

impl DropShadow {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_image(image: DynamicImage) -> Self {
        Self {
            input: Some(ImageIo::Image(image)),
            ..Default::default()
        }
    }

    pub fn from_file(path: String) -> Self {
        Self {
            input: Some(ImageIo::File(path)),
            ..Default::default()
        }
    }

    pub fn margin(&mut self, margin: u32) -> &mut Self {
        self.margin = margin;
        self
    }

    pub fn blur_margin(&mut self, blur_margin: u32) -> &mut Self {
        self.blur_margin = blur_margin;
        self
    }

    pub fn blur_amount(&mut self, blur_amount: f32) -> &mut Self {
        self.blur_amount = blur_amount;
        self
    }

    pub fn input(&mut self, input: ImageIo) -> &mut Self {
        self.input = Some(input);
        self
    }

    pub fn output(&mut self, output: ImageIo) -> &mut Self {
        self.output = Some(output);
        self
    }

    pub fn apply(&self) -> Result<(), ShadowError> {
        // TODO validation
        Err(ShadowError::NotImplemented)
    }
}

impl Default for DropShadow {
    fn default() -> Self {
        DropShadow {
            input: None,
            output: None,

            margin: 20,
            blur_margin: 20,
            blur_amount: 7.0,
        }
    }
}
