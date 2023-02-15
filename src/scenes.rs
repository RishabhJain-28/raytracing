use crate::{bvh::BVH, *};
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
        background_color: None,
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
        background_color: None,
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
        background_color: None,
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
        background_color: None,
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
        background_color: None,
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
pub fn rect_light() -> Scene {
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(26.0, 3.0, 6.0),
        lookat: Point3::new(0.0, 2.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 20.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 16.0 / 9.0,
        background_color: Some(Color::new(0.0, 0.0, 0.0)),
        image_width: 400,
        samples_per_pixel: 400,
        max_depth: 50,
        camera_config,
    });

    let mut world = World::new();

    let mat = Arc::new(Lambertian::new(SolidColor::from_rbg(0.0, 0.0, 0.7)));
    let ground_mat = Arc::new(Lambertian::new(SolidColor::from_rbg(0.7, 0.0, 0.0)));
    let diff_light = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));

    let ground = Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat.clone(),
    ));
    let obj_sphere = Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat.clone()));
    let light_rect = Box::new(Plane::new(
        PlaneOrientation::XY,
        diff_light.clone(),
        0.0,
        4.0,
        0.0,
        4.0,
        -2.0,
    ));
    let light_sphere = Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diff_light.clone(),
    ));
    world.push(obj_sphere);
    world.push(ground);
    world.push(light_rect);
    world.push(light_sphere);

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
        background_color: None,
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

#[allow(dead_code)]
pub fn cornell_box_without_boxes() -> Scene {
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 40.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 1.0,
        background_color: Some(Color::new(0.0, 0.0, 0.0)),
        image_width: 600,
        samples_per_pixel: 400,
        max_depth: 50,
        camera_config,
    });

    let mut world = World::new();

    let red = Arc::new(Lambertian::new(SolidColor::from_rbg(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidColor::from_rbg(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidColor::from_rbg(0.12, 0.45, 0.12)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.push(Box::new(Plane::new(
        PlaneOrientation::YZ,
        green,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::YZ,
        red,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        light.clone(),
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::XY,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);

    (config, world, camera)
}

#[allow(dead_code)]
pub fn cornell_box_scene() -> Scene {
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 40.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 1.0,
        background_color: Some(Color::new(0.0, 0.0, 0.0)),
        image_width: 600,
        samples_per_pixel: 400,
        max_depth: 50,
        camera_config,
    });

    let mut world = World::new();

    let red = Arc::new(Lambertian::new(SolidColor::from_rbg(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidColor::from_rbg(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidColor::from_rbg(0.12, 0.45, 0.12)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.push(Box::new(Plane::new(
        PlaneOrientation::YZ,
        green,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::YZ,
        red,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        light.clone(),
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::XY,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));

    let box1 = Box::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    // let box1 = Box::new(Cube::new(
    //     Point3::new(130.0, 0.0, 65.0),
    //     Point3::new(295.0, 165.0, 230.0),
    //     white.clone(),
    // ));
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(Box::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.push(Box::new(box1));

    let box2 = Box::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(Box::new(box2), Vec3::new(130.0, 0.0, 65.0));
    world.push(Box::new(box2));

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);

    (config, world, camera)
}

#[allow(dead_code)]
pub fn cornell_box_scene_with_smoke_boxes() -> Scene {
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 40.0,
        dist_to_focus: None,
        time0: None,
        time1: None,
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 1.0,
        background_color: Some(Color::new(0.0, 0.0, 0.0)),
        image_width: 600,
        samples_per_pixel: 400,
        max_depth: 50,
        camera_config,
    });

    let mut world = World::new();

    let red = Arc::new(Lambertian::new(SolidColor::from_rbg(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidColor::from_rbg(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidColor::from_rbg(0.12, 0.45, 0.12)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.push(Box::new(Plane::new(
        PlaneOrientation::YZ,
        green,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::YZ,
        red,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        light.clone(),
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    world.push(Box::new(Plane::new(
        PlaneOrientation::XY,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));

    let box1 = Box::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(Box::new(box1), Vec3::new(265.0, 0.0, 295.0));
    let box1 = ConstantMedium::from_color(Box::new(box1), Color::new(0.0, 0.0, 0.0), 0.01);
    world.push(Box::new(box1));

    let box2 = Box::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(Box::new(box2), Vec3::new(130.0, 0.0, 65.0));
    let box2 = ConstantMedium::from_color(Box::new(box2), Color::new(1.0, 1.0, 1.0), 0.01);

    world.push(Box::new(box2));

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);

    (config, world, camera)
}

#[allow(dead_code)]
pub fn ray_tracing_the_next_week() -> Scene {
    let camera_config = CameraConfig::new(CameraConfigOptions {
        lookfrom: Point3::new(478., 278., -600.),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        vfov: 40.0,
        dist_to_focus: None,
        time0: Some(0.0),
        time1: Some(1.0),
    });

    let config = Config::new(ConfigOptions {
        aspect_ratio: 1.0,
        background_color: Some(Color::new(0.0, 0.0, 0.0)),
        image_width: 600,
        samples_per_pixel: 10000,
        max_depth: 50,
        camera_config,
    });

    let mut world = World::new();

    // let ground_mat = Arc::new(Lambertian::new(SolidColor::from_rbg(0.48, 0.83, 0.53)));
    // let boxes_per_side = 20;
    // let mut rng = rand::thread_rng();

    // let mut boxes: HittableList = Vec::new();

    // for i in 0..boxes_per_side {
    //     for j in 0..boxes_per_side {
    //         let w = 100.0;
    //         let x0 = -1000.0 + (i as f64) * w;
    //         let z0 = -1000.0 + (j as f64) * w;
    //         let y0 = 0.0;
    //         let x1 = x0 + w;
    //         let y1 = rng.gen_range(1.0..101.0);
    //         let z1 = z0 + w;
    //         boxes.push(Box::new(Cube::new(
    //             Point3::new(x0, y0, z0),
    //             Point3::new(x1, y1, z1),
    //             ground_mat.clone(),
    //         )))
    //     }
    // }
    // world.push(Box::new(BVH::new(boxes, 0.0, 1.0)));

    let light_mat = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.push(Box::new(Plane::new(
        PlaneOrientation::ZX,
        light_mat,
        147.,
        412.,
        123.,
        423.,
        554.,
    )));
    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Arc::new(Lambertian::new(SolidColor::from_rbg(0.7, 0.3, 0.1)));

    world.push(Box::new(MovingSphere::new(
        50.,
        moving_sphere_material,
        center1,
        center2,
        config.camera_config.time0,
        config.camera_config.time1,
    )));

    world.push(Box::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0)),
    )));

    let boundary = Box::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    let boundary_c = Box::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.push(boundary);
    world.push(Box::new(ConstantMedium::from_color(
        boundary_c,
        Color::new(0.2, 0.4, 0.9),
        0.2,
    )));

    let boundary = Box::new(Sphere::new(
        Point3::new(0., 0., 0.),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    world.push(Box::new(ConstantMedium::from_color(
        boundary,
        Color::new(1., 1., 1.),
        0.0001,
    )));

    let earth_surface = Arc::new(Lambertian::new(ImageTexture::from_file("earthmap.jpg")));
    world.push(Box::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.0,
        earth_surface,
    )));

    // let perlin = Arc::new(Lambertian::new(NoiseTexture::new(0.1)));
    // world.push(Box::new(Sphere::new(
    //     Point3::new(220., 280., 300.),
    //     80.0,
    //     perlin,
    // )));

    // let mut boxes: HittableList = Vec::new();
    // let white = Arc::new(Lambertian::new(SolidColor::from_rbg(0.73, 0.73, 0.73)));
    // let ns = 1000;
    // for _ in 0..ns {
    //     boxes.push(Box::new(Sphere::new(
    //         Point3::random(0.0..165.0),
    //         10.0,
    //         white.clone(),
    //     )))
    // }

    // let boxes_r = RotateY::new(Box::new(BVH::new(boxes, 0.0, 1.0)), 15.0);
    // let boxes_t = Translate::new(Box::new(boxes_r), Vec3::new(-100., 270., 395.));

    // world.push(Box::new(boxes_t));

    let camera = Camera::new(&config.camera_config, config.aspect_ratio);

    (config, world, camera)
}
