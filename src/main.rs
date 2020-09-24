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
use colour::{GREEN, WHITE};
use intersectable::Intersectables;
use material::{LightMaterial, Material, SimpleMaterial};
use scene::Scene;
use sphere::Sphere;

use std::fs::File;
use std::{io::prelude::*, rc::Rc};

// Features:
// =========
// [X] Fix aspect ratio
// [X] Support "HDR"
// [X] Sky box
// [X] Add light sources
// [ ] Load scene from file
// [X] Add indirect light
// [ ] Add plane primitive
// [ ] Add mesh primitive
//     [ ] Add triangle primitive
// [ ] Implement reflection
// [ ] Implement refraction
// [ ] Add sub-pixel rays
// [ ] Support linear -> sRGB colour space (http://chilliant.blogspot.com.au/2012/08/srgb-approximations-for-hlsl.html)

pub fn main() {
    println!("The rusty path tracer!");

    let camera = make_camera();
    let material_white = Rc::new(SimpleMaterial {
        colour: WHITE,
        secondary_rays: 8,
    });

    let material_light = Rc::new(LightMaterial { colour: GREEN });

    let root = Intersectables {
        intersectables: vec![
            Box::new(Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                3.0,
                material_white.clone(),
            )),
            Box::new(Sphere::new(
                Point3::new(2.5, 2.5, 2.5),
                1.0,
                material_light.clone(),
            )),
            Box::new(Sphere::new(
                Point3::new(-2.5, -2.5, 2.5),
                1.0,
                material_light.clone(),
            )),
            Box::new(Sphere::new(
                Point3::new(0.0, 0.0, 3.2),
                0.1,
                material_light.clone(),
            )),
        ],
    };

    let scene = Scene {
        root_intersectable: Box::new(root),
    };

    let width = 1024;
    let height = 1024;
    let image = render(&camera, &scene, width, height);

    let file_create_handle = File::create("image.ppm");
    if let Ok(mut file) = file_create_handle {
        file.write_all(image.as_ref()).unwrap();
    }
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
        .into_iter()
        .map(|ray| scene.get_colour(&ray));

    ppm_image::write_ppm_image(width, height, 255, image)
}
