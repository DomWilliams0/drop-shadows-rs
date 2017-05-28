use image::ImageError;
use std::{io, fmt};
use std::error::Error;

#[derive(Debug)]
pub enum ShadowError {
    Configuration(String),
    Image(ImageError),
    Io(io::Error),
    NotImplemented,
}

impl Error for ShadowError {
    fn description(&self) -> &str {
        match *self {
            ShadowError::Configuration(_) => "configuration error",
            ShadowError::Image(ref err) => err.description(),
            ShadowError::Io(ref err) => err.description(),
            ShadowError::NotImplemented => "not implemented",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ShadowError::Image(ref err) => err.cause(),
            ShadowError::Io(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl fmt::Display for ShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShadowError::Configuration(ref msg) => write!(f, "Configuration error: {}", msg),
            ShadowError::Image(ref err) => err.fmt(f),
            ShadowError::Io(ref err) => err.fmt(f),
            ShadowError::NotImplemented => write!(f, "Not currently implemented"),
        }
    }
}

impl From<io::Error> for ShadowError {
    fn from(err: io::Error) -> ShadowError {
        ShadowError::Io(err)
    }
}

impl From<ImageError> for ShadowError {
    fn from(err: ImageError) -> ShadowError {
        ShadowError::Image(err)
    }
}
