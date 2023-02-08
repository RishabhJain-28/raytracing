use crate::*;
use rand::Rng;
use std::sync::Arc;
pub type Scene = (Config, World, Camera);

#[allow(dead_code)]
pub fn get_random_spheres_scene() -> Scene {
    eprintln!("Random spheres scene!");
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 20.0,
        dist_to_focus: Some(10.0),
        time0: Some(0.0),
        time1: Some(1.0),
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 3.0 / 2.0,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,
        camera_config,
    });

    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(SolidColor::from_rbg(0.5, 0.5, 0.5)));
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

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(SolidColor::new(albedo)));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

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

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(SolidColor::from_rbg(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);

    return (config, world, camera);
}

#[allow(dead_code)]

// add motion
pub fn base_scene() -> Scene {
    eprintln!("Base Scene!");
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(0.0, 0.0, 0.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.0,
        vfov: 90.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 16.0 / 9.0,
        image_width: 256,
        samples_per_pixel: 100,
        max_depth: 5,
        camera_config,
    });
    let camera = Camera::new(&config.camera_config, config.aspect_ratio);
    let mut world = World::new();
    let mat_ground = Arc::new(Lambertian::new(SolidColor::from_rbg(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(SolidColor::from_rbg(0.1, 0.2, 0.5)));
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

#[allow(dead_code)]

pub fn base_scene_without_motion() -> Scene {
    eprintln!("Base Scene!");
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(0.0, 0.0, 0.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.0,
        vfov: 90.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 16.0 / 9.0,
        image_width: 256,
        samples_per_pixel: 100,
        max_depth: 5,
        camera_config,
    });
    let camera = Camera::new(&config.camera_config, config.aspect_ratio);
    let mut world = World::new();
    let mat_ground = Arc::new(Lambertian::new(SolidColor::from_rbg(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(SolidColor::from_rbg(0.1, 0.2, 0.5)));
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

#[allow(dead_code)]
pub fn two_checkered_spheres() -> Scene {
    eprintln!("");
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 20.0,
        dist_to_focus: Some(10.0),
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        camera_config,
    });

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);
    let mut world = World::new();

    let checkered_texture = CheckerTexture::new(
        SolidColor::from_rbg(0.2, 0.3, 0.1),
        SolidColor::from_rbg(0.9, 0.9, 0.9),
    );

    let checkered_material = Arc::new(Lambertian::new(checkered_texture));

    let sphere1 = Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checkered_material.clone(),
    );
    let sphere2 = Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checkered_material.clone(),
    );

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));

    return (config, world, camera);
}

#[allow(dead_code)]
pub fn two_perlin_spheres() -> Scene {
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 20.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        camera_config,
    });

    let mut world = World::new();
    let mat = Arc::new(Lambertian::new(NoiseTexture::new(4.0)));
    // let mat = Arc::new(Lambertian::new(CheckerTexture::new(
    //     SolidColor::from_rbg(0.5, 0.5, 0.5),
    //     SolidColor::from_rbg(0.2, 0.2, 0.2),
    // )));
    let sphere = Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat.clone(),
    ));
    let sphere2 = Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat));
    world.push(sphere);
    world.push(sphere2);

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);

    (config, world, camera)
}
#[allow(dead_code)]
pub fn earth_map_sphere() -> Scene {
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 20.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        camera_config,
    });

    let mut world = World::new();
    let texture = ImageTexture::from_file("earthmap.jpg");
    let earth_surface = Arc::new(Lambertian::new(texture));
    let sphere = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    ));
    world.push(sphere);

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);

    (config, world, camera)
}
