use image::ImageResult as Result;
use std::cmp::{max, min};
use std::path::Path;

use image::{load_from_memory, ImageBuffer, Rgba, RgbaImage};
use lodepng;

#[derive(Clone, Copy)]
pub struct Pixl {
    rgb: [u8; 4],
}

#[derive(Clone)]
pub struct Image {
    img: RgbaImage,
}

impl Pixl {
    pub fn new(r: u8, g: u8, b: u8) -> Pixl {
        Pixl {
            rgb: [r, g, b, 255],
        }
    }

    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Pixl {
        Pixl { rgb: [r, g, b, a] }
    }

    pub fn black() -> Pixl {
        Pixl::new(0, 0, 0)
    }

    pub fn red() -> Pixl {
        Pixl::new(255, 0, 0)
    }

    pub fn invert(&mut self) {
        self.rgb[0] = 255 - self.rgb[0];
        self.rgb[1] = 255 - self.rgb[1];
        self.rgb[2] = 255 - self.rgb[2];
    }
}

impl Image {
    fn pixel_white() -> Rgba<u8> {
        Rgba::<u8>([255, 255, 255, 255])
    }

    pub fn from_png(v: Vec<u8>) -> Option<Image> {
        match load_from_memory(&v) {
            Err(_) => None,
            Ok(i) => Some(Image { img: i.to_rgba8() }),
        }
    }

    pub fn new(w: u32, h: u32) -> Image {
        Image {
            img: ImageBuffer::from_pixel(w, h, Self::pixel_white()),
        }
    }

    pub fn set_color(&mut self, color: &[u8; 4]) {
        // TODO: optimize
        for y in 0..self.img.height() {
            for x in 0..self.img.width() {
                let c = *self.img.get_pixel(x, y);
                if c[0] == 0 {
                    // if red channel is 0 we assume it's a black pixel
                    let rgb = Rgba::<u8>(*color);
                    self.img.put_pixel(x, y, rgb);
                }
            }
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, p: Pixl) {
        if x < self.img.width() && y < self.img.height() {
            self.img.put_pixel(x, y, Rgba::<u8>(p.rgb));
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixl {
        let p = *self.img.get_pixel(x, y);
        Pixl {
            rgb: [p[0], p[1], p[2], p[3]],
        }
    }

    pub fn width(&self) -> u32 {
        self.img.width()
    }

    pub fn height(&self) -> u32 {
        self.img.height()
    }

    pub fn save(&self, p: &Path) -> Result<()> {
        self.img.save(p)
    }

    pub fn draw_polygon(&mut self, poly: &[imageproc::point::Point<i32>], color: Rgba<u8>) {
        imageproc::drawing::draw_polygon_mut(&mut self.img, poly, color);
    }

    pub fn draw_line_segment(&mut self, p1: (f32, f32), p2: (f32, f32), color: Rgba<u8>) {
        imageproc::drawing::draw_line_segment_mut(&mut self.img, p1, p2, color);
    }

    pub fn fill_circle(&mut self, x: u32, y: u32, r: u32, p: Pixl) {
        let h = self.height();
        let w = self.width();

        for py in max(y as i32 - r as i32, 0)..min(y + r, h) as i32 {
            for px in max(x as i32 - r as i32, 0)..min(x + r, w) as i32 {
                let dy = y as i32 - py;
                let dx = x as i32 - px;
                let d = ((dy * dy + dx * dx) as f32).sqrt() as u32;
                if d <= r {
                    self.put_pixel(px as u32, py as u32, p);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.img = ImageBuffer::from_pixel(self.width(), self.height(), Self::pixel_white())
    }

    pub fn add_image(&mut self, x: u32, y: u32, i: &Image) {
        for iy in 0..i.height() {
            for ix in 0..i.width() {
                self.put_pixel(x + ix, y + iy, i.get_pixel(ix, iy));
            }
        }
    }

    pub fn as_png(&self) -> Option<Vec<u8>> {
        let w = self.img.width() as usize;
        let h = self.img.height() as usize;
        let i = self.img.clone().into_raw();

        let mut buf = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));

        match image::write_buffer_with_format(
            &mut buf,
            &i,
            w as u32,
            h as u32,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        ) {
            Err(_) => None,
            Ok(_) => {
                let img = buf.into_inner().unwrap().into_inner();
                Some(img)
            }
        }

        /*match lodepng::encode_memory(&i, w, h, lodepng::ColorType::RGBA, 8) {
            Err(_) => None,
            Ok(v) => Some(v),
        }*/
    }
}
