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
use colour::{Colour, BLACK};
use command_line_options::CommandLineOptions;
use intersectable::Intersectable;
use material::*;
use ray::Ray;
use scene::Scene;
use std::{
    fs,
    fs::File,
    io::Write,
    sync::{mpsc, Arc},
    thread::{self, JoinHandle},
};
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

    let scene = Arc::new(Scene::new(5, root, Box::new(material_skybox.clone())));

    let renderer = Renderer { scene };

    let image = renderer.render(
        &camera,
        command_line_options.width,
        command_line_options.height,
    );

    let file_create_handle = File::create("image.ppm");
    if let Ok(mut file) = file_create_handle {
        file.write_all(image.as_ref()).unwrap();
    }

    // let statistics = &scene.clone().statistics;
    // let statistics = statistics.lock().expect("failed to acquire statistics");

    // println!(
    //     "Number of rays traced: {0}",
    //     statistics.total_number_of_rays_cast
    // );
    // println!(
    //     "Number of rays killed: {0}",
    //     statistics.total_number_of_rays_killed
    // );
}

fn make_camera() -> Camera {
    let origin = Point3::new(0.0, 0.0, 5.0);
    let forward = Vector3::new(0.0, 0.0, -1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let fov = core::f32::consts::PI * 0.5;

    Camera::new(origin, forward, up, fov)
}

struct Renderer {
    scene: Arc<Scene>,
}

struct PixelJob {
    index: usize,
    ray: Ray,
}

unsafe impl Sync for PixelJob {}

#[derive(Debug)]
struct PixelResult {
    index: usize,
    color: Colour,
}

unsafe impl Sync for PixelResult {}
unsafe impl Send for PixelResult {}

impl Renderer {
    // pub fn render(&self, camera: &Camera, width: usize, height: usize) -> String {
    //     let rays: Vec<Ray> = camera.get_viewport(width, height).collect();

    //     let image: Vec<Colour> = rays
    //         // .par_iter()
    //         .iter()
    //         .map(|ray| self.scene.cast_ray(&ray, 0))
    //         .collect();

    //     ppm_image::write_ppm_image(width, height, 255, image.into_iter())
    // }

    pub fn render(&self, camera: &Camera, width: usize, height: usize) -> String {
        let (mut work_tx, work_rx) = spmc::channel::<PixelJob>();
        let (result_tx, result_rx) = mpsc::channel::<PixelResult>();

        let mut worker_threads = Vec::<JoinHandle<()>>::new();
        for _ in 0..12 {
            let thread_work = work_rx.clone();
            let thread_result = result_tx.clone();
            let scene = self.scene.clone();

            worker_threads.push(thread::spawn(move || loop {
                match thread_work.recv() {
                    Ok(job) => {
                        let color = scene.cast_ray(&job.ray, 0);
                        let pixel = PixelResult {
                            color,
                            index: job.index,
                        };
                        thread_result
                            .send(pixel)
                            .expect("the main thread died before all workers completed");
                    }
                    Err(_) => {
                        break;
                    }
                }
            }));
        }

        let viewport = camera.get_viewport(width, height).enumerate();
        viewport.for_each(|job| {
            let pixel_job = PixelJob {
                index: job.0,
                ray: job.1,
            };
            work_tx.send(pixel_job).expect("all workers have died");
        });

        drop(work_tx);

        let image_size = width * height;
        let mut image = vec![BLACK; image_size];

        for _ in 0..image_size {
            let pixel = result_rx.recv().unwrap();
            image[pixel.index] = pixel.color;
        }

        for thread_handle in worker_threads {
            thread_handle.join().expect("failed to join thread");
        }

        ppm_image::write_ppm_image(width, height, 255, image.into_iter())
    }
}
