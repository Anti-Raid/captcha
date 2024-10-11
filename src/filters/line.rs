use super::SerdeColor;
use filters::Filter;
use images::Image;

/// Draw lines/rectangles on the screen
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    p1: (f32, f32),
    p2: (f32, f32),
    thickness: f32,
    color: SerdeColor,
}

impl Line {
    pub fn new(p1: (f32, f32), p2: (f32, f32), thickness: f32, color: SerdeColor) -> Line {
        Line {
            p1,
            p2,
            thickness: {
                if thickness < 1.0 {
                    1.0
                } else {
                    thickness
                }
            },
            color,
        }
    }
}

#[typetag::serde]
impl Filter for Line {
    fn apply(&self, i: &mut Image) -> Result<(), super::Error> {
        let pixl = self.color.to_pixl();

        // Translate pt to center of image
        let center = (i.width() as f32 / 2.0, i.height() as f32 / 2.0);

        let p1 = (self.p1.0 + center.0, self.p1.1 + center.1);

        let p2 = (self.p2.0 + center.0, self.p2.1 + center.1);

        for pt in imageproc::drawing::BresenhamLineIter::new(p1, p2) {
            // Draw the thicknesses
            for y in
                (pt.1 as i32 - self.thickness as i32 / 2)..(pt.1 as i32 + self.thickness as i32 / 2)
            {
                for x in (pt.0 as i32 - self.thickness as i32 / 2)
                    ..(pt.0 as i32 + self.thickness as i32 / 2)
                {
                    if x >= 0 && x < i.width() as i32 && y >= 0 && y < i.height() as i32 {
                        i.put_pixel(x as u32, y as u32, pixl);
                    }
                }
            }
        }

        Ok(())
    }

    fn validate(&self, viewbox: (u32, u32)) -> Result<(), super::Error> {
        if self.thickness < 1.0 {
            return Err("thickness must be greater than 1".into());
        }

        if self.p1 == self.p2 {
            return Err("p1 and p2 must be different".into());
        }

        if self.p1.0.abs() >= viewbox.0 as f32
            || self.p1.1.abs() >= viewbox.1 as f32
            || self.p2.0.abs() >= viewbox.0 as f32
            || self.p2.1.abs() >= viewbox.1 as f32
        {
            return Err("p1 and p2 must be within the viewbox".into());
        }

        Ok(())
    }
}
