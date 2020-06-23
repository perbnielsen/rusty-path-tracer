mod hit;
mod intersectable;
mod ray;
mod sphere;

use cgmath::{vec3, Point3};
use ray::Ray;

pub fn main() {
    println!("The rusty path tracer!");

    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), vec3(1.0, 2.0, 3.0));
    println!("{:?}", ray);
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
