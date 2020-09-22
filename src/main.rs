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
use cgmath::{Basis3, Point3, Rotation, Vector3};
use colour::BLUE;
use material::{Material, SimpleMaterial};
use scene::Scene;
use sphere::Sphere;

use std::fs::File;
use std::{io::prelude::*, rc::Rc};

// Features:
// =========
// [X] Fix aspect ratio
// [X] Support "HDR"
// [X] Sky box
// [ ] Add light sources
// [ ] Load scene from file
// [ ] Add indirect light
// [ ] Add plane primitive
// [ ] Add mesh primitive
//     [ ] Add triangle primitive
// [ ] Implement reflection
// [ ] Implement refraction
// [ ] Add sub-pixel rays
// [ ] Support linear -> sRGB colour space (http://chilliant.blogspot.com.au/2012/08/srgb-approximations-for-hlsl.html)

pub fn main() {
    println!("The rusty path tracer!");

    let width = 1024; //640;
    let height = 1024; //480;

    let origin = Point3::new(0.0, 0.0, 5.0);
    let forward: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let fov = core::f32::consts::PI * 0.5;
    let basis = Basis3::look_at(forward, up);
    let camera = Camera { basis, origin, fov };
    let material = Rc::new(SimpleMaterial { colour: BLUE });
    let scene = Scene {
        root_intersectable: Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 3.0, material)),
    };

    render(&camera, &scene, width, height);
}

fn render(camera: &Camera, scene: &Scene, width: usize, height: usize) {
    let image = camera
        .get_viewport(width, height)
        .into_iter()
        .map(|ray| scene.get_colour(&ray));
    let ppm_image = ppm_image::write_ppm_image(width, height, 255, image);

    let file_create_handle = File::create("image.ppm");

    if let Ok(mut file) = file_create_handle {
        file.write_all(ppm_image.as_ref()).unwrap();
    }
}
