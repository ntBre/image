use super::{Encoder, Rgba};
use png::ColorType;
use std::{error::Error, fs::File, io::BufWriter, ops::Range, path::Path};

pub struct Image {
    pub(crate) width: u32,
    #[allow(unused)]
    pub(crate) height: u32,
    pub(crate) data: Vec<u8>,
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

    pub(crate) fn index(&self, x: usize, y: usize) -> Range<usize> {
        let row = 4 * x * self.width as usize;
        let col = 4 * y;
        row + col..row + col + 4
    }

    pub fn at(&self, x: usize, y: usize) -> Rgba {
        let v = &self.data[self.index(x, y)];
        Rgba::from(v)
    }

    /// set the pixel at row `x` and col `y` to `color`
    pub fn set(&mut self, x: usize, y: usize, color: Rgba) {
        let idx = self.index(x, y);
        self.data[idx].copy_from_slice(&color.as_array());
    }

    /// write self to the PNG file specified by `path`
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), impl Error> {
        let w = BufWriter::new(File::create(path)?);
        let encoder = Encoder::new(w, self.width, self.height);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)
    }
}
