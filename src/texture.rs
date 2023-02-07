use crate::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color;
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
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color {
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
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color {
        let sines =
            f64::sin(10.0 * point.x()) * f64::sin(10.0 * point.y()) * f64::sin(10.0 * point.z());
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}