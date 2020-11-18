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

use colour::Colour;
use intersectable::{Intersectables, Triangle};
use json::iterators::Members;
use material::*;
use scene::Scene;
use sphere::Sphere;

use std::{collections::HashMap, fs, fs::File, io::Write, rc::Rc};

// Features:
// =========
// [X] Fix aspect ratio
// [X] Support 'HDR'
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

    // let scene = read_scene_from_json("scene.json");
    let camera = make_camera();
    let material_mirror = Rc::new(MirrorMaterial {
        colour: colour::LIGHT_BLUE,
    });
    let material_light = Rc::new(LightMaterial {
        colour: colour::LIGHT_GREEN,
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

#[allow(dead_code, unused_variables)]
fn read_scene_from_json(filename: &str) -> HashMap<String, Rc<dyn Material>> {
    let file = fs::read_to_string(filename).expect("Failed to read scene file");
    let json = json::parse(file.as_str()).expect("Failed to parse json file");
    // println!('Materials:\n{}', json['materials']);
    let colours = parse_colours(json["colours"].members());
    let materials = parse_materials(json["materials"].members(), colours);

    materials
    // for material in json['materials'].members() {
    //     let material =

    //     println!('Type: {}', material['name'])
    // }
}

#[allow(dead_code, unused_variables)]
fn parse_colours(colour_definitions: Members) -> HashMap<&str, Colour> {
    let mut colours = HashMap::new();
    for colour_definition in colour_definitions {
        let name = colour_definition["name"]
            .as_str()
            .expect("failed to parse colour name");
        let colour = Colour {
            r: colour_definition["r"]
                .as_f32()
                .expect("failed to parse colour r value"),
            g: colour_definition["g"]
                .as_f32()
                .expect("failed to parse colour g value"),
            b: colour_definition["b"]
                .as_f32()
                .expect("failed to parse colour b value"),
            a: 1.0,
        };
        colours.insert(name, colour);
    }
    colours
}

#[allow(dead_code, unused_variables)]
fn parse_materials(
    material_definitions: Members,
    colours: HashMap<&str, Colour>,
) -> HashMap<String, Rc<dyn Material>> {
    let mut materials = HashMap::<String, Rc<dyn Material>>::new();
    for material_definition in material_definitions {
        let name = material_definition["name"]
            .as_str()
            .expect("failed to parse the material name");
        let material_type = material_definition["type"]
            .as_str()
            .expect("failed to parse the material type");
        match material_type {
            "DiffuseMaterial" => {
                let colour_name = material_definition["colour"]
                    .as_str()
                    .expect("failed to parse colour name");
                let colour = colours.get(&colour_name).unwrap().clone();
                let material = Rc::new(DiffuseMaterial {
                    colour,
                    secondary_rays: 16,
                });
                materials.insert(name.to_owned(), material);
            }
            _ => {}
        }
    }

    materials
}

#[allow(dead_code, unused_variables)]
fn parse_objects(objects: Members, materials: HashMap<String, Rc<dyn Material>>) -> Scene {
    todo!()
}

#[test]
fn parse_colours_test_success() {
    let colour_json = "{
        \"colours\": 
        [ 
            {
                \"name\": \"light_blue\", \"r\": 0.0, \"g\": 0.25, \"b\": 0.75
            }
        ] 
    }\n";
    let json = json::parse(colour_json).expect("Failed to parse json file");
    let colours = parse_colours(json["colours"].members());

    assert!(colours.contains_key("light_blue"));
    assert_eq!(colours["light_blue"].r, 0.0);
    assert_eq!(colours["light_blue"].g, 0.25);
    assert_eq!(colours["light_blue"].b, 0.75);
    assert_eq!(colours["light_blue"].a, 1.0);
}

#[test]
fn parse_materials_test_success() {
    let mut colours = HashMap::new();
    colours.insert("light_blue", colour::LIGHT_BLUE);
    let materials_json = "{
        \"materials\": 
        [
            {
                \"name\": \"diffuse_blue\",
                \"type\": \"DiffuseMaterial\",
                \"colour\": \"light_blue\"
            }
        ]
    }\n";
    let json = json::parse(materials_json).expect("Failed to parse json file");
    let materials = parse_materials(json["materials"].members(), colours);

    assert!(materials.contains_key("diffuse_blue"));
}

#[test]
fn parse_scene_test() {
    let scene = read_scene_from_json("scene.json");
    todo!()
}
