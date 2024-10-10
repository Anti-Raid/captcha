use filters::Filter;
use images::Image;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorInvert {}

impl ColorInvert {
    pub fn new() -> ColorInvert {
        ColorInvert {}
    }
}

#[typetag::serde]
impl Filter for ColorInvert {
    fn apply(&self, i: &mut Image) {
        for y in 0..i.height() {
            for x in 0..i.width() {
                let mut p = i.get_pixel(x, y);
                p.invert();
                i.put_pixel(x, y, p);
            }
        }
    }
}
