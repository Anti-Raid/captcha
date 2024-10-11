extern crate captcha;

use captcha::filters::{Cow, Noise, Wave};
use captcha::{Captcha, Geometry};

use std::path::Path;
fn main() {
    let mut c = Captcha::new();
    println!("{:?}", c.supported_chars());

    c.set_chars(&['a', 'b'])
        .add_random_chars(5)
        .apply_filter(Noise::new(0.2))
        .expect("Noise filter failed")
        .apply_filter(Wave::new(2.0, 20.0))
        .expect("Wave filter failed")
        .view(220, 120)
        .apply_filter(
            Cow::new()
                .min_radius(40)
                .max_radius(50)
                .circles(1)
                .area(Geometry::new(40, 150, 50, 70)),
        )
        .expect("Cow filter failed")
        .set_color([255, 128, 0, 255]);
    c.save(Path::new("captcha.png")).expect("save failed");

    println!(
        "CAPTCHA with text {} written to captcha.png",
        c.chars_as_string()
    );
}
