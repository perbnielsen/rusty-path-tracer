use assert_approx_eq::assert_approx_eq;
use cgmath::{vec3, Point3, Vector3};
#[macro_use]
extern crate matches;

pub fn main() {
    println!("The rusty path tracer!");

    let ray = Ray {
        origin: Point3::new(0.0, 0.0, 0.0),
        direction: vec3(1.0, 2.0, 3.0),
    };
    println!("{:?}", ray);
}

#[derive(Debug)]
struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }
}

#[derive(Debug)]
struct Sphere {
    centre: Point3<f32>,
    radius: f32,
    // material: Box<dyn Material>,
}

impl Sphere {
    fn new(centre: Point3<f32>, radius: f32) -> Self {
        Self { centre, radius }
    }
}

fn vector_from_to(p1: Point3<f32>, p2: Point3<f32>) -> Vector3<f32> {
    Vector3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z)
}

#[test]
fn test_sphere_intersection() {
    let sphere = Sphere {
        centre: Point3::new(0.0, 0.0, 0.0),
        radius: 5.0,
    };
    let ray = Ray {
        origin: Point3::new(0.0, 0.0, -6.0),
        direction: Vector3::new(0.0, 0.0, 1.0),
    };

    let hit = sphere.intersect(&ray);
    assert_matches!(hit, Some(_));

    let hit = hit.unwrap();
    assert_approx_eq!(hit.distance, 1.0);
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let m = vector_from_to(self.centre, ray.origin);
        let b = cgmath::dot(m, ray.direction);
        let c = cgmath::dot(m, m) - self.radius * self.radius;

        // Exit if r’s origin outside s (c > 0) and r pointing away from s (hitB > 0)
        if c > 0.0 && b > 0.0 {
            return None;
        }

        // // Exit if r’s origin outside s (c > 0) and r pointing away from s (hitB > 0)
        let discriminant = (b * b) - c;
        if discriminant < 0.0 {
            return None;
        }

        let t = -b - discriminant.sqrt();
        if t < 0.0 {
            return None;
        }

        let intersection_point = ray.origin + (t * ray.direction);

        Some(Hit::new(
            t,
            intersection_point,
            vector_from_to(self.centre, intersection_point),
            // self.material.clone(),
        ))
    }
}

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Debug)]
struct Hit {
    distance: f32,
    position: Point3<f32>,
    normal: Vector3<f32>,
    // material: Box<dyn Material>,
}

impl Hit {
    fn new(distance: f32, position: Point3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            distance,
            position,
            normal,
            // material,
        }
    }
}

#[derive(Debug)]
struct Colour {}

#[derive(Debug)]
struct Camera {}

#[derive(Debug)]
struct Viewport {}

trait Shader {}

struct SimpleMaterial {
    colour: Colour,
    shader: Box<dyn Shader>,
}

impl Material for SimpleMaterial {
    fn get_colour(&self, view_direction: Vector3<f32>, normal: Vector3<f32>) -> Colour {
        todo!()
    }
}

trait Material {
    fn get_colour(&self, view_direction: Vector3<f32>, normal: Vector3<f32>) -> Colour;
}
