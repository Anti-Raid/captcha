use std::f64::consts;

use filters::Filter;
use images::Image;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    HORIZONTAL,
    VERTICAL,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Direction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Direction::HORIZONTAL => s.serialize_str("horizontal"),
            Direction::VERTICAL => s.serialize_str("vertical"),
        }
    }
}

#[cfg(feature = "serde")]
/// Serde deserialization for LockdownMode
impl<'de> serde::Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Direction, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "horizontal" => Ok(Direction::HORIZONTAL),
            "vertical" => Ok(Direction::VERTICAL),
            _ => Err(serde::de::Error::custom("invalid direction")),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wave {
    f: f64,
    amp: f64,
    d: Direction,
}

impl Wave {
    pub fn new(f: f64, amp: f64) -> Wave {
        Wave {
            f,
            amp,
            d: Direction::HORIZONTAL,
        }
    }

    pub fn horizontal(self) -> Wave {
        Wave {
            d: Direction::HORIZONTAL,
            ..self
        }
    }

    pub fn vertical(self) -> Wave {
        Wave {
            d: Direction::VERTICAL,
            ..self
        }
    }

    pub fn direction(self, d: Direction) -> Wave {
        Wave { d, ..self }
    }
}

// TODO randomize offset
#[typetag::serde]

impl Filter for Wave {
    fn apply(&self, i: &mut Image) -> Result<(), super::Error> {
        let o = i.clone();
        i.clear();
        match self.d {
            Direction::HORIZONTAL => {
                // height of image changes
                for x in 0..i.width() {
                    let f =
                        (x as f64 * 2.0 * consts::PI * self.f / i.width() as f64).sin() * self.amp;
                    for y in 0..i.height() {
                        let ny = y as i32 - f as i32;
                        if ny >= 0 && ny < i.height() as i32 {
                            i.put_pixel(x, ny as u32, o.get_pixel(x, y));
                        }
                    }
                }
            }
            Direction::VERTICAL => {
                for y in 0..i.height() {
                    let f =
                        (y as f64 * 2.0 * consts::PI * self.f / i.width() as f64).sin() * self.amp;
                    for x in 0..i.width() {
                        let nx = x as i32 - f as i32;
                        if nx >= 0 && nx < i.width() as i32 {
                            i.put_pixel(nx as u32, y, o.get_pixel(x, y));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn validate(&self, _viewbox: (u32, u32)) -> Result<(), super::Error> {
        if self.f < 0.0 || self.amp < 0.0 {
            return Err("f and amp must be greater than 0".into());
        }

        if self.f >= 65535.0 || self.amp >= 65535.0 {
            return Err("f and amp must be less than 65535.0 (u16::MAX)".into());
        }

        Ok(())
    }
}
