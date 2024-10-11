//! Filters to disturb and transform CAPTCHAs.

mod color_invert;
mod cow;
mod dots;
mod grid;
mod line;
mod noise;
mod wave;

use images::Image;

// reexports
pub use filters::color_invert::ColorInvert;
pub use filters::cow::Cow;
pub use filters::dots::Dots;
pub use filters::grid::Grid;
pub use filters::line::Line;
pub use filters::noise::Noise;
pub use filters::wave::Wave;

#[typetag::serde(tag = "filter")]
pub trait Filter {
    fn apply(&self, i: &mut Image);
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerdeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl SerdeColor {
    pub fn new(r: u8, g: u8, b: u8) -> SerdeColor {
        SerdeColor { r, g, b }
    }

    pub fn to_pixl(&self) -> crate::images::Pixl {
        crate::images::Pixl::new(self.r, self.g, self.b)
    }
}
