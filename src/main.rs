mod camera;
mod colour;
mod hit;
mod intersectable;
mod ppm_image;
mod ray;
mod sphere;
mod viewport;

use camera::Camera;
use cgmath::{Basis3, Point3, Rotation, Vector3};
use colour::Colour;
use hit::Hit;
use intersectable::Intersectable;
use sphere::Sphere;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    println!("The rusty path tracer!");

    render();
}

fn render() {
    let width = 640;
    let height = 480;

    let origin = Point3::new(0.0, 0.0, 5.0);
    let forward: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let fov = core::f32::consts::PI * 0.5;
    let basis = Basis3::look_at(forward, up);
    let camera = Camera { basis, origin, fov };
    let scene = Sphere::new(Point3::new(0.0, 0.0, 0.0), 3.0);
    let image = camera
        .get_viewport(width, height)
        .into_iter()
        .map(|ray| get_colour(scene.intersect(&ray)));
    let ppm_image = ppm_image::write_ppm_image(width, height, 255, image);

    let file = File::create("image.ppm");

    if let Ok(mut file) = file {
        file.write_all(ppm_image.as_ref());
    }
}

const GREY: Colour = Colour {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};

const BLUE: Colour = Colour {
    r: 0.25,
    g: 0.25,
    b: 1.0,
    a: 1.0,
};

fn get_colour(hit: Option<Hit>) -> Colour {
    match hit {
        Some(_) => BLUE,
        None => GREY,
    }
}
// trait Shader {}

// struct SimpleMaterial {
//     colour: Colour,
//     shader: Box<dyn Shader>,
// }

// impl Material for SimpleMaterial {
//     fn get_colour(&self, view_direction: Vector3<f32>, normal: Vector3<f32>) -> Colour {
//         todo!()
//     }
// }

// trait Material {
//     fn get_colour(&self, view_direction: Vector3<f32>, normal: Vector3<f32>) -> Colour;
// }
