use super::{Encoder, Rgba};
use png::ColorType;
use std::{error::Error, fs::File, io::BufWriter, ops::Range, path::Path};

pub mod draw {
    use crate::{Image, Rgba};

    pub trait Draw {
        type Endpoint;

        /// draw `self` onto `img` at (`x`, `y`) in `color`
        fn draw(
            &self,
            img: &mut Image,
            x: Self::Endpoint,
            y: Self::Endpoint,
            color: Rgba,
        );
    }

    pub struct Line {
        width: usize,
    }

    impl Line {
        pub fn new(width: usize) -> Self {
            Self { width }
        }
    }

    impl Draw for Line {
        type Endpoint = (usize, usize);

        fn draw(
            &self,
            img: &mut Image,
            x: Self::Endpoint,
            y: Self::Endpoint,
            color: Rgba,
        ) {
            let (fx, fy) = x;
            let (tx, ty) = y;
            // TODO bounds checks. also these should all be f64 so -ve is okay
            let m = (ty as f64 - fy as f64) / (tx as f64 - fx as f64);
            let b = ty as f64 - m * tx as f64;
            let f = |x| (m * x as f64 + b).round() as usize;
            let hw = self.width / 2;
            for i in fx..tx {
                for w in i - hw..i + hw {
                    img.set(f(i), w, color);
                }
            }
        }
    }

    pub struct Circle {
        radius: usize,
    }

    impl Circle {
        pub fn new(radius: usize) -> Self {
            Self { radius }
        }
    }

    impl Draw for Circle {
        type Endpoint = usize;

        fn draw(&self, img: &mut Image, x: usize, y: usize, color: Rgba) {
            let r = self.radius;
            let (w, h) = img.shape();
            // bounds-checks
            let lx = if x < r { 0 } else { x - r };
            let ly = if y < r { 0 } else { y - r };
            let hx = if x + r >= w - 1 { w - 1 } else { x + r };
            let hy = if y + r >= h - 1 { h - 1 } else { y + r };
            let mx = (lx + hx) / 2;
            let my = (ly + hy) / 2;
            for i in lx..=hx {
                for j in ly..=hy {
                    let ix = if i > mx { i - mx } else { mx - i };
                    let iy = if j > my { j - my } else { my - j };
                    if ix * ix + iy * iy <= r * r {
                        img.set(j, i, color);
                    }
                }
            }
        }
    }

    pub struct Square {
        length: usize,
    }

    impl Draw for Square {
        type Endpoint = usize;

        fn draw(&self, img: &mut Image, x: usize, y: usize, color: Rgba) {
            let r = self.length;
            let (w, h) = img.shape();
            // bounds-checks
            let lx = if x < r { 0 } else { x - r };
            let ly = if y < r { 0 } else { y - r };
            let hx = if x + r >= w - 1 { w - 1 } else { x + r };
            let hy = if y + r >= h - 1 { h - 1 } else { y + r };
            for i in lx..=hx {
                for j in ly..=hy {
                    img.set(i, j, color);
                }
            }
        }
    }
}

pub struct Image {
    pub(crate) width: usize,
    #[allow(unused)]
    pub(crate) height: usize,
    pub(crate) data: Vec<u8>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; 4 * height * width],
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
            width: info.width as usize,
            height: info.height as usize,
            data,
        })
    }

    /// return the width and height of the image
    pub fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// fill `self` with `color`
    pub fn fill(&mut self, color: Rgba) {
        for chunk in self.data.chunks_mut(4) {
            chunk.copy_from_slice(&color.as_array());
        }
    }

    pub(crate) fn index(&self, x: usize, y: usize) -> Range<usize> {
        let row = 4 * x * self.width;
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
