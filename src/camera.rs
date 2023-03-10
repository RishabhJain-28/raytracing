use rand::Rng;

use crate::{CameraConfig, Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(camera_config: &CameraConfig, aspect_ratio: f64) -> Camera {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * camera_config.vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (camera_config.lookfrom - camera_config.lookat).normalized();
        let cu = camera_config.vup.cross(cw).normalized();
        let cv = cw.cross(cu);
        let h = camera_config.dist_to_focus * viewport_width * cu;
        let v = camera_config.dist_to_focus * viewport_height * cv;

        let llc = camera_config.lookfrom - h / 2.0 - v / 2.0 - camera_config.dist_to_focus * cw;

        Camera {
            origin: camera_config.lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            cu: cu,
            cv: cv,
            lens_radius: camera_config.aperture / 2.0,
            time0: camera_config.time0,
            time1: camera_config.time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.cu * rd.x() + self.cv * rd.y();

        let mut time = self.time0;
        if self.time1 - self.time0 > f64::EPSILON {
            let mut rng = rand::thread_rng();
            time = rng.gen_range(self.time0..self.time1);
        }

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
