mod camera;
mod colour;
mod command_line_options;
mod hit;
mod intersectable;
mod material;
mod ppm_image;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod viewport;

use crate::renderer::Renderer;
use camera::Camera;

use command_line_options::CommandLineOptions;
use intersectable::Intersectable;
use material::*;

use scene::Scene;

use sdl2::keyboard::{Keycode, Scancode};
use sdl2::{event::Event, pixels::PixelFormatEnum};
use std::fs::File;
use std::io::Write;
use std::{fs, time::Instant};
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
// [X] Parallel rendering
//   [X] Use bigger jobs?
// [X] Realtime UI
// [ ] Add plane primitive
// [ ] Add mesh primitive
// [ ] Implement refraction
// [ ] Add sub-pixel rays
// [ ] Support linear -> sRGB colour space (http://chilliant.blogspot.com.au/2012/08/srgb-approximations-for-hlsl.html)
// [ ] Convert to library
// [ ] Run firegraph to see bottle-necks

pub fn load_scene(file_name: String) -> Scene {
    let file = fs::read_to_string(file_name).expect("Failed to read scene");
    let root: Box<dyn Intersectable> =
        serde_json::from_str(file.as_str()).expect("Failed to parse scene");
    let material_skybox = SkyBoxMaterial {
        colour_top: colour::LIGHT_BLUE,
        colour_bottom: colour::WHITE,
    };

    Scene::new(5, root, Box::new(material_skybox.clone()))
}

pub fn main() {
    let command_line_options = CommandLineOptions::from_args();
    let window_width = command_line_options.width;
    let window_height = command_line_options.height;

    let scene = load_scene(command_line_options.scene);
    let camera = Camera::default();
    let renderer = Renderer {
        num_workers: command_line_options.num_workers,
        num_chunks: command_line_options.num_chunks,
        scene,
    };

    if command_line_options.real_time_ui {
        real_time_ui(window_width, window_height, camera, renderer);
    } else {
        render_image_to_file(
            renderer,
            camera,
            window_width,
            window_height,
            command_line_options.image_name,
        );
    }
}

fn render_image_to_file(
    renderer: Renderer,
    camera: Camera,
    width: usize,
    height: usize,
    image_name: String,
) {
    let image = renderer.render(&camera, width, height);
    let now = Instant::now();

    println!("Writing image... ({}ms)", now.elapsed().as_millis());

    let image_string = ppm_image::write_ppm_image(width, height, image);

    println!("Done... ({}ms)", now.elapsed().as_millis());

    if let Ok(mut file) = File::create(image_name) {
        file.write_all(image_string.as_ref())
            .expect("failed to write image to file");
    }
}

fn real_time_ui(window_width: usize, window_height: usize, mut camera: Camera, renderer: Renderer) {
    let sdl_context = sdl2::init().expect("failed to initialise the sdl context");
    let video_subsystem = sdl_context
        .video()
        .expect("failed to initialise the video subsystem");
    let window = video_subsystem
        .window(
            "rust-sdl2 demo",
            2 * window_width as u32,
            2 * window_height as u32,
        )
        .position_centered()
        .build()
        .expect("failed to build the window");
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .expect("failed to build renderer");
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            window_width as u32,
            window_height as u32,
        )
        .expect("failed to crete streaming texture");
    let mut event_pump = sdl_context
        .event_pump()
        .expect("failed to acquire event pump");
    let mut last_frame_start_time = Instant::now();

    'running: loop {
        let delta_time = last_frame_start_time.elapsed().as_secs_f32();
        last_frame_start_time = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            camera.translate(camera.left() * delta_time);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            camera.translate(-camera.left() * delta_time);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Up) {
            camera.translate(camera.up() * delta_time);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Down) {
            camera.translate(-camera.up() * delta_time);
        }
        if keyboard_state.is_scancode_pressed(Scancode::PageDown) {
            camera.translate(camera.forward() * delta_time);
        }
        if keyboard_state.is_scancode_pressed(Scancode::PageUp) {
            camera.translate(-camera.forward() * delta_time);
        }

        texture
            .with_lock(None, |pixels, _stride| {
                let image = renderer.render(&camera, window_width, window_height);
                let mut i = 0;
                for pixel in image {
                    pixels[i] = (pixel.r * 255.0) as u8;
                    i = i + 1;
                    pixels[i] = (pixel.g * 255.0) as u8;
                    i = i + 1;
                    pixels[i] = (pixel.b * 255.0) as u8;
                    i = i + 1;
                }
            })
            .expect("failed to acquire texture lock");

        canvas.copy(&texture, None, None).expect("copy failed");
        canvas.present();
    }
}
