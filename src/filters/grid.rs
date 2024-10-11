use filters::Filter;
use images::{Image, Pixl};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Grid {
    y_gap: u32,
    x_gap: u32,
}

impl Grid {
    pub fn new(x_gap: u32, y_gap: u32) -> Grid {
        Grid { x_gap, y_gap }
    }
}

#[typetag::serde]
impl Filter for Grid {
    fn apply(&self, i: &mut Image) -> Result<(), super::Error> {
        for y in (0..i.height()).filter(|i| i % self.y_gap == 0) {
            for x in 0..i.width() {
                i.put_pixel(x, y, Pixl::black());
            }
        }
        for x in (0..i.width()).filter(|i| i % self.x_gap == 0) {
            for y in 0..i.height() {
                i.put_pixel(x, y, Pixl::black());
            }
        }

        Ok(())
    }

    fn validate(&self, viewbox: (u32, u32)) -> Result<(), super::Error> {
        if self.x_gap <= 0
            || self.y_gap <= 0
            || self.x_gap >= viewbox.0
            || self.x_gap >= viewbox.1
            || self.y_gap >= viewbox.0
            || self.y_gap >= viewbox.1
        {
            return Err(
                "x_gap and y_gap must be greater than 0 and must be smaller than the viewbox"
                    .into(),
            );
        }

        Ok(())
    }
}
