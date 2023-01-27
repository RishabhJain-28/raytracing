use crate::{Point3, Ray, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
