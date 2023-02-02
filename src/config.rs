use crate::{Point3, Vec3};

pub struct ConfigOptions {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: u64,
    pub max_depth: u64,
    pub camera_config: CameraConfig,
}

pub struct Config {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub image_height: u64,
    pub samples_per_pixel: u64,
    pub max_depth: u64,
    pub camera_config: CameraConfig,
}

impl Config {
    pub fn new(options: ConfigOptions) -> Self {
        Self {
            aspect_ratio: options.aspect_ratio,
            image_width: options.image_width,
            image_height: ((options.image_width as f64) / options.aspect_ratio) as u64,
            samples_per_pixel: options.samples_per_pixel,
            max_depth: options.max_depth,
            camera_config: options.camera_config,
        }
    }
}

pub struct CameraConfigOptions {
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub aperture: f64,
    pub vfov: f64,
    pub dist_to_focus: Option<f64>,
    pub time0: Option<f64>,
    pub time1: Option<f64>,
}
pub struct CameraConfig {
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub dist_to_focus: f64,
    pub aperture: f64,
    pub vfov: f64,
    pub time0: f64,
    pub time1: f64,
}
impl CameraConfig {
    pub fn new(config: CameraConfigOptions) -> Self {
        let dist_to_focus = config
            .dist_to_focus
            .unwrap_or_else(|| (config.lookfrom - config.lookat).length());
        Self {
            lookfrom: config.lookfrom,
            dist_to_focus,
            lookat: config.lookat,
            vup: config.vup,
            aperture: config.aperture,
            vfov: config.vfov,
            time0: config.time0.unwrap_or_default(),
            time1: config.time0.unwrap_or_default(),
        }
    }
}
