extern crate image;

pub mod error;
pub use error::ShadowError;

use image::DynamicImage;
use std::path::Path;

pub enum ImageInput<'a> {
    Image(&'a DynamicImage),
    File(&'a Path),
}

pub struct DropShadowBuilder<'a> {
    margin: u32,
    blur_margin: u32,
    blur_amount: f32,

    input: Option<ImageInput<'a>>,
}

pub struct DropShadow {
    image: DynamicImage,
}

impl<'a> DropShadowBuilder<'a> {
    pub fn from_image(image: &'a DynamicImage) -> Self {
        Self {
            input: Some(ImageInput::Image(image)),
            ..Default::default()
        }
    }

    pub fn from_file(path: &'a Path) -> Self {
        Self {
            input: Some(ImageInput::File(path)),
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

    pub fn input(&'a mut self, input: ImageInput<'a>) -> &'a mut Self {
        self.input = Some(input);
        self
    }

    pub fn apply(&self) -> Result<DropShadow, ShadowError> {
        // TODO validation
        Err(ShadowError::NotImplemented)
    }
}

impl DropShadow {
    pub fn to_file(&self, path: &Path) -> Result<(), ShadowError> {
        Err(ShadowError::NotImplemented)
    }
}

impl<'a> Default for DropShadowBuilder<'a> {
    fn default() -> Self {
        Self {
            input: None,

            margin: 20,
            blur_margin: 20,
            blur_amount: 7.0,
        }
    }
}
