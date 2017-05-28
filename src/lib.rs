extern crate image;

mod error;
pub use error::ShadowError;

use image::{DynamicImage, GenericImage, ImageFormat};
use std::path::Path;
use std::fs;
use std::ops::Deref;

pub type ShadowResult<T> = Result<T, ShadowError>;

pub enum ImageInput<'a> {
    Image(&'a DynamicImage),
    File(&'a Path),
}

pub struct DropShadowBuilder<'a> {
    config: Config,
    input: ImageInput<'a>,
}

#[derive(Copy, Clone)]
pub struct Config {
    pub margin: u32,
    pub blur_margin: u32,
    pub blur_amount: f32,
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

    pub fn config(&mut self, config: &Config) -> &mut Self {
        self.config = config.clone();
        self
    }

    pub fn input(&mut self, input: ImageInput<'a>) -> &mut Self {
        self.input = input;
        self
    }

    pub fn apply(&mut self) -> ShadowResult<DropShadow> {
        let image = match self.input {
            ImageInput::Image(image) => self.apply_drop_shadow(image),
            ImageInput::File(path) => self.apply_drop_shadow(&image::open(path)?),
        }?;

        Ok(DropShadow { image: image })
    }

    fn validate(&mut self, dimensions: Tuple) {
        let dimensions = (dimensions.0 + self.config.margin * 2,
                          dimensions.1 + self.config.margin * 2);
        let max = std::cmp::max(dimensions.0, dimensions.1);

        // range checks
        if self.config.blur_margin > self.config.margin + max {
            self.config.blur_margin = self.config.margin + max;
        }

        if dimensions.1 < 2 * self.config.margin + 2 * self.config.blur_margin {
            self.config.blur_margin = (dimensions.1 - 2 * self.config.margin) / 2;
        }
    }

    fn apply_drop_shadow(&mut self, image: &DynamicImage) -> ShadowResult<DynamicImage> {
        self.validate(image.dimensions());
        apply_drop_shadow(image, &self.config)
    }
}

impl DropShadow {
    pub fn to_file(&self, path: &Path) -> ShadowResult<()> {
        let mut out_file = fs::File::create(path)?;
        self.image.save(&mut out_file, ImageFormat::PNG)?;

        Ok(())
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

fn apply_drop_shadow(image: &DynamicImage, config: &Config) -> ShadowResult<DynamicImage> {

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
        if !resized.copy_from(&view, edge.paste_pos.0, edge.paste_pos.1) {
            return Err(ShadowError::Image(String::from("Failed to apply an unrotated edge")));
        }

        let rot = image::imageops::rotate180(&view);
        if !resized.copy_from(&rot,
                              edge.paste_pos.0 + edge.rotated_offset.0,
                              edge.paste_pos.1 + edge.rotated_offset.1) {
            return Err(ShadowError::Image(String::from("Failed to apply a rotated edge")));
        }
    }

    // copy original image across
    resized.copy_from(image, config.margin, config.margin);

    Ok(resized)
}
