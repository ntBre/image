use std::fmt::Debug;

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
