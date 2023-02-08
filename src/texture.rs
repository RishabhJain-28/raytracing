use crate::{Color, Perlin, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color;
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self {
            color_value: Color::from(color),
        }
    }

    pub fn from_rbg(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        return self.color_value;
    }
}

#[derive(Clone, Copy)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    odd: T,
    even: U,
}
impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(odd: T, even: U) -> Self {
        Self { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color {
        let sines =
            f64::sin(10.0 * point.x()) * f64::sin(10.0 * point.y()) * f64::sin(10.0 * point.z());
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

pub struct NoiseTexture {
    perlin_noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin_noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, point: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * point.z() + 10.0 * self.perlin_noise.turb(point, None)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_file(file: &str) -> Self {
        let img = image::open(file)
            .expect("imgage for texture not found")
            .to_rgb8();
        let (w, h) = img.dimensions();
        let data = img.into_raw();
        Self {
            data,
            width: w,
            height: h,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Point3) -> Color {
        let w = self.width as usize;
        let h = self.height as usize;
        let mut i = (u * w as f64) as usize;
        let mut j = ((1.0 - v) * h as f64) as usize;
        if i >= w {
            i = w - 1
        }
        if j >= h {
            j = h - 1
        }
        let index = 3 * i + 3 * w * j;
        let r = self.data[index] as f64 / 255.0;
        let g = self.data[index + 1] as f64 / 255.0;
        let b = self.data[index + 2] as f64 / 255.0;
        Color::new(r, g, b)
    }
}
