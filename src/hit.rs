use std::sync::Arc;

use crate::{aabb, material::Material, Point3, Ray, Vec3, AABB};

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

pub struct Translate {
    obj: Box<dyn Hitable>,
    offest: Vec3,
}

impl Translate {
    pub fn new(obj: Box<dyn Hitable>, offest: Vec3) -> Self {
        Self { obj, offest }
    }
}
impl Hitable for Translate {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let bbox = self.obj.bounding_box(time0, time1)?;

        Some(aabb::AABB::new(
            bbox.min() + self.offest,
            bbox.max() + self.offest,
        ))
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offest, r.direction(), r.time());

        let mut rec = self.obj.hit(&moved_r, t_min, t_max)?;

        rec.p += self.offest;
        rec.set_face_normal(&moved_r, rec.normal);

        Some(rec)
    }
}

pub struct RotateY {
    obj: Box<dyn Hitable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn bbox(&self) -> Option<AABB> {
        self.bbox
    }
    pub fn new(obj: Box<dyn Hitable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // change from unwrap
        let bbox = match obj.bounding_box(0.0, 1.0) {
            Some(bbox) => bbox,
            None => {
                return Self {
                    bbox: None,
                    cos_theta,
                    sin_theta,
                    obj,
                };
            }
        };

        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            obj,
            sin_theta,
            cos_theta,
            bbox: Some(AABB::new(min, max)),
        }
    }
}

impl Hitable for RotateY {
    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        self.bbox()
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated = Ray::new(origin, direction, r.time());

        let mut rec = self.obj.hit(&rotated, t_min, t_max)?;

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated, normal);

        Some(rec)
    }
}
