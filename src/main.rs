use rand::Rng;
use ray_tracing_in_one_weekend::{
    base_scene, get_random_spheres_scene, Camera, CameraConfig, Color, Config, Dielectric, Hit,
    Lambertian, Metal, Point3, Ray, Scene, Sphere, Vec3, World,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    io::{stderr, Write},
    rc::Rc,
    sync::Arc,
};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
fn generate_dev_config() -> Config {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let vfov = 20.0;
    Config {
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        camera_config: CameraConfig {
            lookfrom,
            lookat,
            vup,
            dist_to_focus,
            aperture,
            vfov,
        },
    }
}
fn dev_scene() -> Scene {
    eprintln!("test scene!");
    let config = generate_dev_config();

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);
    let mut world = World::new();
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left.clone());
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    return (config, world, camera);
}

fn main() {
    let (config, world, camera) = get_random_spheres_scene();

    println!("P3");
    println!("{} {}", config.image_width, config.image_height);
    println!("255");

    for j in (0..config.image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j + 1);
        let scanline: Vec<Color> = (0..config.image_width)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..config.samples_per_pixel {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((config.image_width - 1) as f64);
                    let v = ((j as f64) + random_v) / ((config.image_height - 1) as f64);

                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, config.max_depth);
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(config.samples_per_pixel));
        }
    }
    eprintln!("\nDone.");
}

fn main_seq_executor() {
    let (config, world, camera) = base_scene();

    println!("P3");
    println!("{} {}", config.image_width, config.image_height);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..config.image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j + 1);
        for i in 0..config.image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..config.samples_per_pixel {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((config.image_width - 1) as f64);
                let v = ((j as f64) + random_v) / ((config.image_height - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, config.max_depth);
            }

            println!("{}", pixel_color.format_color(config.samples_per_pixel));
        }
    }
    eprintln!("\nDone.");
}
