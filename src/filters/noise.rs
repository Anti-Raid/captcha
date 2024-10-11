use rand::{thread_rng, Rng};

use filters::Filter;
use images::{Image, Pixl};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Noise {
    prob: f32,
}

impl Noise {
    pub fn new(prob: f32) -> Noise {
        Noise { prob }
    }
}

#[typetag::serde]
impl Filter for Noise {
    fn apply(&self, i: &mut Image) -> Result<(), super::Error> {
        let mut rng = thread_rng();
        for y in 0..i.height() {
            for x in 0..i.width() {
                if rng.gen::<f32>() <= self.prob {
                    i.put_pixel(x, y, Pixl::black());
                }
            }
        }

        Ok(())
    }

    fn validate(&self, _viewbox: (u32, u32)) -> Result<(), super::Error> {
        if self.prob < 0.0 || self.prob > 1.0 {
            return Err("prob must be between 0.0 and 1.0".into());
        }

        Ok(())
    }
}
