extern crate captcha;

use captcha::filters::{ColorInvert, Cow, Grid, Line, Noise, SerdeColor, Wave};
use captcha::{Captcha, Geometry};

use std::path::Path;

fn main() {
    let mut c = Captcha::new();
    c.add_random_chars(8)
        .apply_filter(Noise::new(0.05))
        .apply_filter(Wave::new(4.0, 20.0))
        .apply_filter(Grid::new(10, 30))
        .apply_filter(Line::new(
            (0.0, 0.0),
            (30.0, 100.0),
            10.0,
            SerdeColor::new(0, 0, 0),
        ))
        .apply_filter(ColorInvert::new())
        .view(280, 160)
        .apply_filter(
            Cow::new()
                .min_radius(40)
                .max_radius(50)
                .circles(1)
                .area(Geometry::new(40, 150, 50, 70)),
        )
        .apply_filter(
            Cow::new()
                .min_radius(80)
                .max_radius(100)
                .circles(1)
                .area(Geometry::new(40, 150, 100, 140)),
        );
    c.save(Path::new("captcha.png")).expect("save failed");

    println!(
        "CAPTCHA with text {} written to captcha.png",
        c.chars_as_string()
    );
}
