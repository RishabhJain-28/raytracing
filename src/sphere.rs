use crate::{
    hit::{Hit, HitRecord},
    Point3,
};

pub struct Sphere {
    radius: f64,
    center: Point3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { radius, center }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = r.direction().dot(oc);
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        Some(HitRecord {
            p,
            t: root,
            normal: (p - self.center) / self.radius,
        })
    }
}
