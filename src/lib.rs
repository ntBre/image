//! a package for manipulating images

use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

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

pub struct Rgba(u8, u8, u8, u8);

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

pub struct Image {
    width: u32,
    #[allow(unused)]
    height: u32,
    data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (4 * height * width) as usize],
        }
    }

    /// decode a PNG image from the file denoted by `path`
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let decoder = png::Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;

        let mut buf = vec![0; reader.output_buffer_size()];
        let frame = reader.next_frame(&mut buf)?;
        // Grab the bytes of the image.
        let bytes = &buf[..frame.buffer_size()];

        assert!(frame.color_type == ColorType::Rgb);
        let mut data = Vec::with_capacity(bytes.len() * 4 / 3);
        for chunk in bytes.chunks_exact(3) {
            data.extend(chunk);
            data.push(255);
        }

        let info = reader.info();
        Ok(Self {
            width: info.width,
            height: info.height,
            data,
        })
    }

    /// fill `self` with `color`
    pub fn fill(&mut self, color: Rgba) {
        for chunk in self.data.chunks_mut(4) {
            chunk.copy_from_slice(&color.as_array());
        }
    }

    /// set the pixel at row `x` and col `y` to `color`
    pub fn set(&mut self, x: usize, y: usize, color: Rgba) {
        let v = color.as_array();
        let row = 4 * x * self.width as usize;
        let col = 4 * y;
        self.data[row + col..row + col + 4].copy_from_slice(&v);
    }

    /// write self to the PNG file specified by `path`
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), impl Error> {
        let w = BufWriter::new(File::create(path)?);
        let encoder = Encoder::new(w, self.width, self.height);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)
    }
}
