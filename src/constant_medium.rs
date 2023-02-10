use std::sync::Arc;

use crate::{Color, Hitable, Isotropic, Material, SolidColor, Texture};

pub struct ConstantMedium<T: Texture> {
    boundary: Box<dyn Hitable>,
    phase_function: Arc<Isotropic<T>>,
    new_inv_density: f64,
}

impl<T: Texture> ConstantMedium<T> {
    pub fn new(boundary: Box<dyn Hitable>, texture: T, density: f64) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(texture)),
            new_inv_density: (-1.0 / density),
        }
    }
}

impl ConstantMedium<SolidColor> {
    pub fn from_color(boundary: Box<dyn Hitable>, color: Color, density: f64) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::from_color(color)),
            new_inv_density: (-1.0 / density),
        }
    }
}

impl<T: Texture> Hitable for ConstantMedium<T> {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::AABB> {
        self.boundary.bounding_box(time0, time1)
    }
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::HitRecord> {}
}
