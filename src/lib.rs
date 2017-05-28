extern crate image;

mod error;
pub use error::ShadowError;

use image::DynamicImage;
use std::path::Path;
use std::ops::Deref;

pub enum ImageInput<'a> {
    Image(&'a DynamicImage),
    File(&'a Path),
}

pub struct DropShadowBuilder<'a> {
    config: Config,
    input: ImageInput<'a>,
}

struct Config {
    margin: u32,
    blur_margin: u32,
    blur_amount: f32,
}

pub struct DropShadow {
    image: DynamicImage,
}

impl<'a> DropShadowBuilder<'a> {
    pub fn from_image(image: &'a DynamicImage) -> Self {
        Self {
            input: ImageInput::Image(image),
            config: Default::default(),
        }
    }

    pub fn from_file(path: &'a Path) -> Self {
        Self {
            input: ImageInput::File(path),
            config: Default::default(),
        }
    }

    pub fn margin(&mut self, margin: u32) -> &mut Self {
        self.config.margin = margin;
        self
    }

    pub fn blur_margin(&mut self, blur_margin: u32) -> &mut Self {
        self.config.blur_margin = blur_margin;
        self
    }

    pub fn blur_amount(&mut self, blur_amount: f32) -> &mut Self {
        self.config.blur_amount = blur_amount;
        self
    }

    pub fn input(&'a mut self, input: ImageInput<'a>) -> &'a mut Self {
        self.input = input;
        self
    }

    pub fn apply(&self) -> Result<DropShadow, ShadowError> {
        self.validate()?;

        Err(ShadowError::NotImplemented)
    }

    fn validate(&self) -> Result<(), ShadowError> {
        // TODO blur_margin > margin?
        // TODO blur_amount > 0?

        Ok(())
    }
}

impl DropShadow {
    pub fn to_file(&self, path: &Path) -> Result<(), ShadowError> {
        Err(ShadowError::NotImplemented)
    }

    pub fn get_image(self) -> DynamicImage {
        self.image
    }

    pub fn get_image_ref(&self) -> &DynamicImage {
        &self.image
    }
}

impl Deref for DropShadow {
    type Target = DynamicImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            margin: 20,
            blur_margin: 20,
            blur_amount: 7.0,
        }
    }
}
