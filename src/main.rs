use rand::Rng;
use ray_tracing_in_one_weekend::{Camera, Color, Hit, Point3, Ray, Sphere, Vec3, World};
use std::io::{stderr, Write};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
const SAMPLES_PER_PIXEL: u64 = 100;
const MAX_DEPTH: u64 = 5;

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();
        let r = Ray::new(rec.p, target - rec.p);
        0.5 * ray_color(&r, world, depth - 1)
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
fn main() {
    let camera = Camera::new();
    let scene = create_scene();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &scene, MAX_DEPTH);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("\nDone.");
}

fn create_scene() -> World {
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    world
}
