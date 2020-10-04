mod camera;
mod colour;
mod hit;
mod intersectable;
mod material;
mod ppm_image;
mod ray;
mod scene;
mod sphere;
mod viewport;

use camera::Camera;
use cgmath::{Point3, Vector3};

use intersectable::{Intersectables, Triangle};
use material::*;
use scene::Scene;
use sphere::Sphere;

use std::{fs::File, io::Write, rc::Rc};

// Features:
// =========
// [X] Fix aspect ratio
// [X] Support "HDR"
// [X] Sky box
// [X] Add light sources
// [X] Add indirect light
// [X] Add triangle primitive
// [X] Implement reflection
// [ ] Add plane primitive
// [ ] Add mesh primitive
// [ ] Load scene from file
// [ ] Implement refraction
// [ ] Add sub-pixel rays
// [ ] Support linear -> sRGB colour space (http://chilliant.blogspot.com.au/2012/08/srgb-approximations-for-hlsl.html)

pub fn main() {
    println!("The rusty path tracer!");

    let camera = make_camera();
    let material_mirror = Rc::new(MirrorMaterial {
        colour: colour::LIGHT_BLUE,
    });
    let material_light = Rc::new(LightMaterial {
        colour: colour::GREEN,
    });
    let material_checker = Rc::new(CheckerMaterial { grid_size: 0.5 });
    let material_diffuse = Rc::new(DiffuseMaterial {
        colour: colour::LIGHT_GREY,
        secondary_rays: 64,
    });
    let material_skybox = Rc::new(SkyBoxMaterial {
        colour_top: colour::LIGHT_BLUE,
        colour_bottom: colour::WHITE,
    });

    let root = Intersectables {
        intersectables: vec![
            Box::new(Sphere {
                centre: Point3::new(0.0, 0.0, 0.0),
                radius: 2.0,
                material: material_diffuse.clone(),
            }),
            Box::new(Sphere {
                centre: Point3::new(2.5, 2.5, 2.5),
                radius: 1.0,
                material: material_light.clone(),
            }),
            Box::new(Sphere {
                centre: Point3::new(2.5, 0.0, 2.0),
                radius: 1.0,
                material: material_checker.clone(),
            }),
            Box::new(Sphere {
                centre: Point3::new(0.0, -3.0, -1.0),
                radius: 2.0,
                material: material_mirror.clone(),
            }),
            Box::new(Triangle::new(
                Point3::new(0.0, -4.0, 0.0),
                Point3::new(-4.0, -4.0, 0.0),
                Point3::new(-4.0, 0.0, 0.0),
                material_checker.clone(),
            )),
        ],
    };

    let scene = Scene::new(5, Box::new(root), material_skybox.clone());

    let width = 1024;
    let height = 1024;
    let image = render(&camera, &scene, width, height);

    let file_create_handle = File::create("image.ppm");
    if let Ok(mut file) = file_create_handle {
        file.write_all(image.as_ref()).unwrap();
    }

    let statistics = scene.statistics.borrow();

    println!(
        "Number of rays traced: {0}",
        statistics.total_number_of_rays_cast
    );
    println!(
        "Number of rays killed: {0}",
        statistics.total_number_of_rays_killed
    );
}

fn make_camera() -> Camera {
    let origin = Point3::new(0.0, 0.0, 5.0);
    let forward: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let fov = core::f32::consts::PI * 0.5;

    Camera::new(origin, forward, up, fov)
}

fn render(camera: &Camera, scene: &Scene, width: usize, height: usize) -> String {
    let image = camera
        .get_viewport(width, height)
        .map(|ray| scene.cast_ray(&ray, 0));

    ppm_image::write_ppm_image(width, height, 255, image)
}
