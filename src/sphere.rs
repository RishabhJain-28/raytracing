use std::{
    f64::consts::{FRAC_PI_2, PI},
    sync::Arc,
};

use crate::{
    hit::{HitRecord, Hitable},
    material::Material,
    Point3, Vec3, AABB,
};

pub struct Sphere {
    radius: f64,
    center: Point3,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            radius,
            center,
            mat,
        }
    }
    fn get_shpere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
    // fn get_shpere_uv(p: Point3) -> (f64, f64) {
    //     let theta = p.y().asin();
    //     let phi = p.z().atan2(p.x());

    //     let u = 1.0 - (phi + PI) / (2.0 * PI);
    //     let v = (theta + FRAC_PI_2) / PI;

    //     (u, v)
    // }
}

impl Hitable for Sphere {
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

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            u: 0.0,
            v: 0.0,
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        let (u, v) = Sphere::get_shpere_uv(outward_normal);

        rec.set_face_normal(r, outward_normal);
        rec.u = u;
        rec.v = v;

        Some(rec)
    }
    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(
            self.center - radius_vec,
            self.center + radius_vec,
        ))
    }
}

pub struct MovingSphere {
    radius: f64,
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    mat: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        radius: f64,
        mat: Arc<dyn Material>,
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
    ) -> Self {
        Self {
            radius,
            center0,
            center1,
            time0,
            time1,
            mat,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.y()).atan2(p.x()) + PI;

        let u = phi / 2.0 * PI;
        let v = theta / PI;

        (u, v)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
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

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            u: 0.0,
            v: 0.0,
        };

        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        let (u, v) = Sphere::get_shpere_uv(outward_normal);

        rec.set_face_normal(r, outward_normal);
        rec.u = u;
        rec.v = v;

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);

        let box0 = AABB::new(
            self.center(time0) - radius_vec,
            self.center(time0) + radius_vec,
        );
        let box1 = AABB::new(
            self.center(time1) - radius_vec,
            self.center(time1) + radius_vec,
        );

        Some(AABB::surrounding_box(&box0, &box1))
    }
}
