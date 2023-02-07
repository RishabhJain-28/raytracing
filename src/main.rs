use rand::Rng;
use ray_tracing_in_one_weekend::{
    base_scene, two_perlin_spheres, Camera, CameraConfig, CheckerTexture,
    Color, Config, Dielectric, Hitable, Lambertian, Metal, MovingSphere, Point3, Ray, Scene,
    SolidColor, Sphere, Vec3, World,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;

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
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    // let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
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
            time0: 0.0,
            time1: 1.0,
        },
    }
}

#[allow(dead_code)]
fn dev_scene() -> Scene {
    eprintln!("dev scene!");
    let config = generate_dev_config();

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let checkered_texture = CheckerTexture::new(
        SolidColor::from_rbg(0.2, 0.3, 0.1),
        SolidColor::from_rbg(0.9, 0.9, 0.9),
    );

    let ground_mat = Arc::new(Lambertian::new(checkered_texture));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    let sphere_mat = Arc::new(Lambertian::new(SolidColor::new(albedo)));

                    let center0 = center;

                    let center1 = center + Point3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    let sphere = MovingSphere::new(0.2, sphere_mat, center0, center1, 0.0, 1.0);

                    world.push(Box::new(sphere));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random(0.4..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere::new(center, 0.2, sphere_mat);

                    world.push(Box::new(sphere));
                } else {
                    // Glass
                    let sphere_mat = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere::new(center, 0.2, sphere_mat);

                    world.push(Box::new(sphere));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(SolidColor::from_rbg(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    return (config, world, camera);
}

fn main() {
    let (config, world, camera) = two_perlin_spheres();

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

#[allow(dead_code)]
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
