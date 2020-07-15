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

pub fn main() {
    println!("The rusty path tracer!");

    render();
}

fn render() {
    let width = 640;
    let height = 480;

    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 5.0,
    };
    let forward = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let fov = core::f32::consts::PI * 0.5;
    let basis = Basis3::look_at(forward, up);
    let camera = Camera { basis, origin, fov };

    // let scene = ReadSceneFile("/Users/pernielsen/Personal/path-tracer/PathTracer/test_scene.yaml");
    let viewport = camera.get_viewport(width, height);
    let intersectResults = viewport.iter(); //.map().Select(scene.Intersect);
                                            // let imageColours = intersectResults.Select(GetColour);
                                            // let imageString = PpmImage.RenderImage(Width, Height, imageColours, maxColourValue: 255);
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
