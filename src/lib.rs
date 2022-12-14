//! a package for manipulating images

use std::io::Write;

pub use image::*;
pub use rgba::*;

pub mod image;
pub mod rgba;

/// newtype wrapping `png::Encoder` and simplifying the API with more default
/// settings
pub struct Encoder<'a, W: Write>(png::Encoder<'a, W>);

impl<'a, W: Write> Encoder<'a, W> {
    pub fn new(w: W, width: usize, height: usize) -> Self {
        let mut e = png::Encoder::new(w, width as u32, height as u32);
        e.set_color(png::ColorType::Rgba);
        Self(e)
    }

    pub fn write_header(self) -> Result<png::Writer<W>, png::EncodingError> {
        self.0.write_header()
    }
}
