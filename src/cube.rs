use std::sync::Arc;

use crate::{world::HittableList, Hitable, Material, Plane, Point3, AABB};

pub struct Cube {
    min: Point3,
    max: Point3,

    sides: HittableList,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, mat: Arc<dyn Material>) -> Self {
        let mut sides: HittableList = Vec::new();

        sides.push(Box::new(Plane::new(
            crate::PlaneOrientation::XY,
            mat.clone(),
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
        )));
        sides.push(Box::new(Plane::new(
            crate::PlaneOrientation::XY,
            mat.clone(),
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
        )));

        sides.push(Box::new(Plane::new(
            crate::PlaneOrientation::ZX,
            mat.clone(),
            p0.z(),
            p1.z(),
            p0.x(),
            p1.x(),
            p1.y(),
        )));
        sides.push(Box::new(Plane::new(
            crate::PlaneOrientation::ZX,
            mat.clone(),
            p0.z(),
            p1.z(),
            p0.x(),
            p1.x(),
            p0.y(),
        )));
        sides.push(Box::new(Plane::new(
            crate::PlaneOrientation::YZ,
            mat.clone(),
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
        )));
        sides.push(Box::new(Plane::new(
            crate::PlaneOrientation::YZ,
            mat.clone(),
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
        )));

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hitable for Cube {
    fn bounding_box(&self, _: f64, _: f64) -> Option<crate::AABB> {
        let output_box = AABB::new(self.min, self.max);
        Some(output_box)
    }
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
