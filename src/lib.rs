//! a package for manipulating images

use std::{fmt::Debug, io::Write};

pub use image::*;
pub mod image;

/// newtype wrapping `png::Encoder` and simplifying the API with more default
/// settings
pub struct Encoder<'a, W: Write>(png::Encoder<'a, W>);

impl<'a, W: Write> Encoder<'a, W> {
    pub fn new(w: W, width: u32, height: u32) -> Self {
        let mut e = png::Encoder::new(w, width, height);
        e.set_color(png::ColorType::Rgba);
        Self(e)
    }

    pub fn write_header(self) -> Result<png::Writer<W>, png::EncodingError> {
        self.0.write_header()
    }
}

#[derive(Copy, Clone)]
pub struct Rgba(u8, u8, u8, u8);

impl Debug for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02x}{:02x}{:02x}{:02x}",
            self.0, self.1, self.2, self.3
        )
    }
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub const fn red() -> Self {
        Self::new(255, 0, 0, 255)
    }

    pub const fn green() -> Self {
        Self::new(0, 255, 0, 255)
    }

    pub const fn blue() -> Self {
        Self::new(0, 0, 255, 255)
    }

    pub const fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    pub const fn as_array(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

impl From<&[u8]> for Rgba {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() == 4);
        Self(value[0], value[1], value[2], value[3])
    }
}
