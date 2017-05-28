extern crate image;

mod error;
pub use error::ShadowError;

use image::{DynamicImage, GenericImage};
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

    pub fn apply(self) -> Result<DropShadow, ShadowError> {
        self.validate()?;

        let image = match self.input {
            ImageInput::Image(image) => self.apply_drop_shadow(image),
            ImageInput::File(path) => self.apply_drop_shadow(&image::open(path)?),
        }?;

        Ok(DropShadow { image: image })
    }

    fn validate(&self) -> Result<(), ShadowError> {
        // TODO blur_margin > margin?
        // TODO blur_amount > 0?

        Ok(())
    }

    fn apply_drop_shadow(&self, image: &DynamicImage) -> Result<DynamicImage, ShadowError> {
        apply_drop_shadow(image, &self.config)
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

type Tuple = (u32, u32);
struct BlurredEdge {
    pub orig_pos: Tuple,
    pub dims: Tuple,

    pub paste_pos: Tuple,
    pub rotated_offset: Tuple,
}

fn apply_drop_shadow(image: &DynamicImage, config: &Config) -> Result<DynamicImage, ShadowError> {

    // create resized image
    let dims_orig = image.dimensions();
    let mut resized = DynamicImage::new_rgba8(dims_orig.0 + config.margin * 2,
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
        // TODO remove asserts
        assert!(resized.copy_from(&view, edge.paste_pos.0, edge.paste_pos.1));

        let rot = image::imageops::rotate180(&view);
        assert!(resized.copy_from(&rot,
                                  edge.paste_pos.0 + edge.rotated_offset.0,
                                  edge.paste_pos.1 + edge.rotated_offset.1));
    }

    // copy original image across
    resized.copy_from(image, config.margin, config.margin);

    Ok(resized)
}
