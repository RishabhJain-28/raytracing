use std::{f64::INFINITY, sync::Arc};

use rand::Rng;

use crate::{Color, HitRecord, Hitable, Isotropic, Material, SolidColor, Texture, Vec3};

pub struct ConstantMedium<T: Texture> {
    boundary: Box<dyn Hitable>,
    phase_function: Arc<Isotropic<T>>,
    neg_inv_density: f64,
}

impl<T: Texture> ConstantMedium<T> {
    pub fn new(boundary: Box<dyn Hitable>, texture: T, density: f64) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(texture)),
            neg_inv_density: (-1.0 / density),
        }
    }
}

impl ConstantMedium<SolidColor> {
    pub fn from_color(boundary: Box<dyn Hitable>, color: Color, density: f64) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::from_color(color)),
            neg_inv_density: (-1.0 / density),
        }
    }
}

impl<T: Texture + 'static> Hitable for ConstantMedium<T> {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::AABB> {
        self.boundary.bounding_box(time0, time1)
    }
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::HitRecord> {
        let mut rec1 = self.boundary.hit(r, -INFINITY, INFINITY)?;
        let mut rec2 = self.boundary.hit(r, rec1.t + 0.001, INFINITY)?;

        if rec1.t < t_min {
            rec1.t = t_min;
        };

        if rec2.t > t_max {
            rec2.t = t_max;
        };

        if rec1.t >= rec2.t {
            return None;
        };

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        };

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let mut rng = rand::thread_rng();
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().log10();

        if hit_distance > distance_inside_boundary {
            return None;
        };

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        let normal = Vec3::new(1.0, 0.0, 0.0);
        let front_face = true;
        let mat: Arc<Isotropic<T>> = self.phase_function.clone();

        let rec = HitRecord {
            p,
            t,
            front_face,
            normal,
            mat,
            u: 0.0,
            v: 0.0,
        };

        Some(rec)
    }
}
