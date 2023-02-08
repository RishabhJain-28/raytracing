use std::sync::Arc;

use crate::{HitRecord, Hitable, Material, Point3, Vec3, AABB};

pub enum PlaneOrientation {
    XY,
    YZ,
    ZX,
}
pub struct Plane {
    mat: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    plane: PlaneOrientation,
}

impl Plane {
    pub fn new(
        plane: PlaneOrientation,
        mat: Arc<dyn Material>,
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
    ) -> Self {
        Self {
            mat,
            x0,
            x1,
            y0,
            y1,
            k,
            plane,
        }
    }
    fn get_orientation_axis(&self) -> (usize, usize, usize) {
        match self.plane {
            PlaneOrientation::XY => (2, 0, 1),
            PlaneOrientation::YZ => (0, 1, 2),
            PlaneOrientation::ZX => (1, 2, 0),
        }
    }
}

impl Hitable for Plane {
    fn bounding_box(&self, _: f64, _: f64) -> Option<crate::AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::HitRecord> {
        let (k_axis, x_axis, y_axis) = self.get_orientation_axis();

        let x0 = self.x0;
        let x1 = self.x1;
        let y0 = self.y0;
        let y1 = self.y1;
        let t = (self.k - r.origin()[k_axis]) / r.direction()[k_axis];
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin()[x_axis] + t * r.direction()[x_axis];
        let y = r.origin()[y_axis] + t * r.direction()[y_axis];
        if x < x0 || x > x1 || y < y0 || y > y1 {
            return None;
        }

        let mut rec = HitRecord {
            u: (x - x0) / (x1 - x0),
            v: (y - y0) / (y1 - y0),
            mat: self.mat.clone(),
            p: r.at(t),
            t,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let mut outward_normal = Vec3::new(0.0, 0.0, 0.0);
        outward_normal[k_axis] = 1.0;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
