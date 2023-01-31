use rand::Rng;
use ray_tracing_in_one_weekend::{
    Camera, CameraConfig, Color, Dielectric, Hit, Lambertian, Metal, Point3, Ray, Sphere, Vec3,
    World,
};
use std::{
    io::{stderr, Write},
    rc::Rc,
};

// const ASPECT_RATIO: f64 = 16.0 / 9.0;
// const IMAGE_WIDTH: u64 = 256;
// const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
// const SAMPLES_PER_PIXEL: u64 = 100;
// const MAX_DEPTH: u64 = 5;

// const ASPECT_RATIO: f64 = 3.0 / 2.0;
// const IMAGE_WIDTH: u64 = 1200;
// const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
// const SAMPLES_PER_PIXEL: u64 = 500;
// const MAX_DEPTH: u64 = 50;
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
struct Config {
    aspect_ratio: f64,
    image_width: u64,
    image_height: u64,
    samples_per_pixel: u64,
    max_depth: u64,
    camera_config: CameraConfig,
}

impl Config {
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_depth: u64,
        camera_config: CameraConfig,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: ((image_width as f64) / aspect_ratio) as u64,
            samples_per_pixel,
            max_depth,
            camera_config,
        }
    }
}

fn main() {
    // const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_WIDTH: u64 = 256;
    // const SAMPLES_PER_PIXEL: u64 = 100;
    // const MAX_DEPTH: u64 = 5;
    // let lookfrom = Point3::new(3.0, 3.0, 2.0);
    // let lookat = Point3::new(0.0, 0.0, -1.0);
    // let vup = Vec3::new(0.0, 1.0, 0.0);
    // let dist_to_focus = (lookfrom - lookat).length();
    // let aperture = 2.0;

    let camera_config = CameraConfig::new(
        Point3::new(3.0, 3.0, 2.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        2.0,
        20.0,
    );

    let config = Config::new(16.0 / 9.0, 256, 100, 5, camera_config);

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);
    let mut world = World::new();
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left.clone());
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    println!("P3");
    println!("{} {}", config.image_width, config.image_height);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..config.image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        stderr().flush().unwrap();
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

fn glass_ball_scene() {}
// fn create_camera() -> Camera {

//     // let lookfrom = Point3::new(13.0, 2.0, 3.0);
//     // let lookat = Point3::new(0.0, 0.0, 0.0);
//     // let vup = Vec3::new(0.0, 1.0, 0.0);
//     // let dist_to_focus = 10.0;
//     // let aperture = 0.1;
//     // Camera::new(
//     //     lookfrom,
//     //     lookat,
//     //     vup,
//     //     20.0,
//     //     ASPECT_RATIO,
//     //     aperture,
//     //     dist_to_focus,
//     // )
// // }

// fn random_scene() -> World {
//     let mut rng = rand::thread_rng();
//     let mut world = World::new();

//     let ground_mat = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
//     let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

//     world.push(Box::new(ground_sphere));

//     for a in -11..=11 {
//         for b in -11..=11 {
//             let choose_mat: f64 = rng.gen();
//             let center = Point3::new(
//                 (a as f64) + rng.gen_range(0.0..0.9),
//                 0.2,
//                 (b as f64) + rng.gen_range(0.0..0.9),
//             );

//             if choose_mat < 0.8 {
//                 // Diffuse
//                 let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
//                 let sphere_mat = Rc::new(Lambertian::new(albedo));
//                 let sphere = Sphere::new(center, 0.2, sphere_mat);

//                 world.push(Box::new(sphere));
//             } else if choose_mat < 0.95 {
//                 // Metal
//                 let albedo = Color::random(0.4..1.0);
//                 let fuzz = rng.gen_range(0.0..0.5);
//                 let sphere_mat = Rc::new(Metal::new(albedo, fuzz));
//                 let sphere = Sphere::new(center, 0.2, sphere_mat);

//                 world.push(Box::new(sphere));
//             } else {
//                 // Glass
//                 let sphere_mat = Rc::new(Dielectric::new(1.5));
//                 let sphere = Sphere::new(center, 0.2, sphere_mat);

//                 world.push(Box::new(sphere));
//             }
//         }
//     }

//     let mat1 = Rc::new(Dielectric::new(1.5));
//     let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
//     let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

//     let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
//     let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
//     let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

//     world.push(Box::new(sphere1));
//     world.push(Box::new(sphere2));
//     world.push(Box::new(sphere3));

//     world
// }
