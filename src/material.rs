use rand::Rng;

use crate::{Color, HitRecord, Point3, Ray, SolidColor, Texture, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _point: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(a: T) -> Self {
        Lambertian { albedo: a }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());

        Some((self.albedo.value(rec.u, rec.v, rec.p), scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        Metal { albedo: a, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
//

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().normalized();
        let cos_theta = ((-1.0) * unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction, r_in.time());

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn from_color(color: Color) -> Self {
        Self {
            emit: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, point: Point3) -> Color {
        self.emit.value(u, v, point)
    }
}

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Self { albedo: texture }
    }
}

impl Isotropic<SolidColor> {
    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        Some((attenuation, scattered))
    }
}
