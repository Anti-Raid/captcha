extern crate captcha;

use captcha::filters::{Cow, Grid, Noise, Wave};
use captcha::{Captcha, Geometry};

use std::path::Path;

fn main() {
    let mut c = Captcha::new();
    c.add_random_chars(5)
        .apply_filter(Noise::new(0.05))
        .apply_filter(Wave::new(4.0, 20.0))
        .apply_filter(Grid::new(10, 30))
        .view(220, 120)
        .apply_filter(
            Cow::new()
                .min_radius(40)
                .max_radius(50)
                .circles(1)
                .area(Geometry::new(40, 150, 50, 70)),
        );
    c.save(Path::new("captcha.png")).expect("save failed");

    println!(
        "CAPTCHA with text {} written to captcha.png",
        c.chars_as_string()
    );
}
