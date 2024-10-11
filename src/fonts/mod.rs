use base64::decode;
use serde_json;
use std::collections::HashMap;

pub trait Font {
    fn png_as_base64(&self, letter: char) -> Option<&String>;

    fn chars(&self) -> Vec<char>;

    /// Returns None if letter does not exist or if letter could not decoded.
    fn png(&self, letter: char) -> Option<Vec<u8>> {
        match self.png_as_base64(letter) {
            None => None,
            Some(s) => match decode(s) {
                Err(_) => None,
                Ok(v) => Some(v),
            },
        }
    }
}

pub struct Default {
    data: HashMap<char, String>,
}

impl Default {
    /// Create a new default font
    pub fn new() -> Default {
        Self::from_json(include_str!("font_default.json")).expect("Failed to load default font")
    }

    /// Create a new font from a json string
    pub fn from_json(json: &str) -> Result<Default, serde_json::Error> {
        Ok(Default {
            data: serde_json::from_str(json)?,
        })
    }
}

impl Font for Default {
    fn png_as_base64(&self, letter: char) -> Option<&String> {
        self.data.get(&letter)
    }

    fn chars(&self) -> Vec<char> {
        self.data.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use fonts::{Default, Font};
    use images::Image;

    #[test]
    fn fonts_default() {
        let f = Default::new();

        assert_eq!(f.chars().len(), 57);
        assert!(f.png_as_base64('a').is_some());
        assert!(f.png('a').is_some());
        for i in f.chars() {
            assert!(Image::from_png(f.png(i).unwrap()).is_some());
        }
    }
}
