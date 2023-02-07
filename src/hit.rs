use std::sync::Arc;

use crate::{material::Material, Point3, Ray, Vec3, AABB};

pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub mat: Arc<dyn Material>,
    pub normal: Vec3,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}
