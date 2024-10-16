use rand::{thread_rng, Rng};

use filters::Filter;
use images::{Image, Pixl};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dots {
    n: u32,
    min_radius: u32,
    max_radius: u32,
}

impl Dots {
    pub fn new(n: u32) -> Dots {
        Dots {
            n,
            min_radius: 5,
            max_radius: 10,
        }
    }

    pub fn min_radius(self, r: u32) -> Dots {
        Dots {
            min_radius: r,
            ..self
        }
    }

    pub fn max_radius(self, r: u32) -> Dots {
        Dots {
            max_radius: r,
            ..self
        }
    }
}

#[typetag::serde]
impl Filter for Dots {
    fn apply(&self, i: &mut Image) -> Result<(), super::Error> {
        let mut rng = thread_rng();
        for _ in 0..self.n {
            let x = rng.gen_range(0..i.width());
            let y = rng.gen_range(0..i.height());
            let r = rng.gen_range(self.min_radius..self.max_radius + 1);
            i.fill_circle(x, y, r, Pixl::black());
        }

        Ok(())
    }

    fn validate(&self, viewbox: (u32, u32)) -> Result<(), super::Error> {
        if self.min_radius <= 0
            || self.max_radius <= 0
            || self.min_radius >= viewbox.0
            || self.min_radius >= viewbox.1
            || self.max_radius >= viewbox.0
            || self.max_radius >= viewbox.1
        {
            return Err("min_radius and max_radius must be greater than 0 and must be smaller than the viewbox".into());
        }

        if self.n <= 0 || self.n >= 5 {
            return Err("n must be greater than 0 and less than 5".into());
        }

        Ok(())
    }
}
