mod camera;
mod colour;
mod command_line_options;
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
use command_line_options::CommandLineOptions;
use intersectable::Intersectable;
use material::*;
use scene::Scene;
use std::{fs, fs::File, io::Write, rc::Rc};
use structopt::StructOpt;

// Features:
// =========
// [X] Fix aspect ratio
// [X] Support 'HDR'
// [X] Sky box
// [X] Add light sources
// [X] Add indirect light
// [X] Add triangle primitive
// [X] Implement reflection
// [X] Load scene from file
// [ ] Add plane primitive
// [ ] Add mesh primitive
// [ ] Implement refraction
// [ ] Add sub-pixel rays
// [ ] Support linear -> sRGB colour space (http://chilliant.blogspot.com.au/2012/08/srgb-approximations-for-hlsl.html)

pub fn main() {
    let command_line_options = CommandLineOptions::from_args();
    let file = fs::read_to_string(command_line_options.scene).expect("Failed to read scene file");
    let root: Box<dyn Intersectable> =
        serde_json::from_str(file.as_str()).expect("Failed to parse scene file");
    let camera = make_camera();
    let material_skybox = Rc::new(SkyBoxMaterial {
        colour_top: colour::LIGHT_BLUE,
        colour_bottom: colour::WHITE,
    });

    let scene = Scene::new(5, root, material_skybox.clone());

    let image = render(
        &camera,
        &scene,
        command_line_options.width,
        command_line_options.height,
    );

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
