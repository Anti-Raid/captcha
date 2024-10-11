use filters::Filter;
use image::Rgba;
use images::Image;
use rand::thread_rng;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RandomLine {}

impl RandomLine {
    pub fn new() -> RandomLine {
        RandomLine {}
    }

    fn gen_line_color(rng: &mut ThreadRng) -> Rgba<u8> {
        let red = rng.gen_range(100..=255);
        let green = rng.gen_range(100..=255);
        let blue = rng.gen_range(100..=255);
        Rgba([red, green, blue, 255])
    }
}

#[typetag::serde]
impl Filter for RandomLine {
    fn apply(&self, img: &mut Image) -> Result<(), super::Error> {
        let mut rng = thread_rng();

        let line_color = Self::gen_line_color(&mut rng);
        let is_h = rng.gen();
        let (start, end) = if is_h {
            let xa = rng.gen_range(0.0..(img.width() as f32) / 2.0);
            let ya = rng.gen_range(0.0..(img.height() as f32));
            let xb = rng.gen_range((img.width() as f32) / 2.0..(img.width() as f32));
            let yb = rng.gen_range(0.0..(img.height() as f32));
            ((xa, ya), (xb, yb))
        } else {
            let xa = rng.gen_range(0.0..(img.width() as f32));
            let ya = rng.gen_range(0.0..(img.height() as f32) / 2.0);
            let xb = rng.gen_range(0.0..(img.width() as f32));
            let yb = rng.gen_range((img.height() as f32) / 2.0..(img.height() as f32));
            ((xa, ya), (xb, yb))
        };
        let thickness = rng.gen_range(2..4);
        for i in 0..thickness {
            let offset = i as f32;
            if is_h {
                img.draw_line_segment(
                    (start.0, start.1 + offset),
                    (end.0, end.1 + offset),
                    line_color,
                );
            } else {
                img.draw_line_segment(
                    (start.0 + offset, start.1),
                    (end.0 + offset, end.1),
                    line_color,
                );
            }
        }

        Ok(())
    }

    fn validate(&self, _viewbox: (u32, u32)) -> Result<(), super::Error> {
        Ok(())
    }
}
