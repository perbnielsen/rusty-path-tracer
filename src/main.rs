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
use colour::BLACK;
use command_line_options::CommandLineOptions;
use intersectable::Intersectable;
use material::*;
use ray::Ray;
use scene::Scene;
use scoped_threadpool::Pool;
use std::{fs, fs::File, io::Write, time::Instant};
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
// [ ] Parallel rendering
//   [ ] Use bigger jobs?
// [ ] Add plane primitive
// [ ] Add mesh primitive
// [ ] Implement refraction
// [ ] Add sub-pixel rays
// [ ] Support linear -> sRGB colour space (http://chilliant.blogspot.com.au/2012/08/srgb-approximations-for-hlsl.html)

pub fn main() {
    let command_line_options = CommandLineOptions::from_args();
    let file = fs::read_to_string(command_line_options.scene).expect("Failed to read scene");
    let root: Box<dyn Intersectable> =
        serde_json::from_str(file.as_str()).expect("Failed to parse scene");
    let camera = make_camera();
    let material_skybox = SkyBoxMaterial {
        colour_top: colour::LIGHT_BLUE,
        colour_bottom: colour::WHITE,
    };

    let scene = Scene::new(5, root, Box::new(material_skybox.clone()));

    let renderer = Renderer {
        num_workers: command_line_options.num_workers,
        num_chunks: command_line_options.num_chunks,
        scene,
    };

    let image = renderer.render(
        &camera,
        command_line_options.width,
        command_line_options.height,
    );

    let file_create_handle = File::create(command_line_options.image_name);
    if let Ok(mut file) = file_create_handle {
        file.write_all(image.as_ref()).unwrap();
    }
}

fn make_camera() -> Camera {
    let origin = Point3::new(0.0, 0.0, 5.0);
    let forward = Vector3::new(0.0, 0.0, -1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let fov = core::f32::consts::PI * 0.5;

    Camera::new(origin, forward, up, fov)
}

struct Renderer {
    num_workers: usize,
    num_chunks: usize,
    scene: Scene,
}

impl Renderer {
    pub fn render(&self, camera: &Camera, width: usize, height: usize) -> String {
        let image_size = width * height;
        let chunk_size = image_size / self.num_chunks;

        println!("Image     : {} x {} = {}", width, height, image_size);
        println!("Workers   : {}", self.num_workers);
        println!("Chunks    : {}", self.num_chunks);
        println!("Chunk size: {}", chunk_size);

        let mut image = vec![BLACK; image_size];

        println!("Generation rays...");
        let now = Instant::now();

        let rays: Vec<Ray> = camera.get_viewport(width, height).collect();

        println!("Casting rays... ({})", now.elapsed().as_secs());
        let now = Instant::now();

        Pool::new(self.num_workers as u32).scoped(|scope| {
            let ray_chunks = rays.chunks(chunk_size);

            let image_chunks = image.chunks_mut(chunk_size);

            let jobs = ray_chunks.zip(image_chunks);
            for (rays, image_chunk) in jobs {
                scope.execute(move || {
                    rays.iter()
                        .enumerate()
                        .for_each(|(idx, ray)| image_chunk[idx] = self.scene.cast_ray(ray, 0));
                });
            }
        });

        println!("Writing image... ({})", now.elapsed().as_secs());
        let now = Instant::now();

        let image_string = ppm_image::write_ppm_image(width, height, 255, image.into_iter());

        println!("Done... ({})", now.elapsed().as_secs());

        image_string
    }
}
