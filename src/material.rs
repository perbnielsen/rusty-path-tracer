use std::fmt::Debug;

use crate::colour;
use crate::colour::Colour;
use crate::ray::Ray;
use crate::scene::Scene;
use cgmath::InnerSpace;
use cgmath::Point3;
use cgmath::Vector3;
use cgmath::VectorSpace;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[typetag::serde]
pub trait Material: Debug + Sync + Send {
    fn get_colour(
        &self,
        scene: &Scene,
        view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        normal: &Vector3<f32>,
        ray_depth: u8,
    ) -> Colour;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MirrorMaterial {
    pub colour: Colour,
}

#[typetag::serde]
impl Material for MirrorMaterial {
    fn get_colour(
        &self,
        scene: &Scene,
        view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        normal: &Vector3<f32>,
        ray_depth: u8,
    ) -> Colour {
        let view_direction_projected_on_normal = cgmath::dot(*view_direction, *normal) * normal;
        let reflection = view_direction - 2.0 * view_direction_projected_on_normal;
        let ray = Ray {
            origin: position.clone(),
            direction: reflection,
        };

        scene.cast_ray(&ray, ray_depth) * self.colour
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiffuseMaterial {
    pub colour: Colour,
    pub secondary_rays: i32,
}

#[typetag::serde]
impl Material for DiffuseMaterial {
    fn get_colour(
        &self,
        scene: &Scene,
        _view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        normal: &Vector3<f32>,
        ray_depth: u8,
    ) -> Colour {
        let colours = (0..self.secondary_rays).map(|_| {
            let random_direction = unit_vector_in_hemisphere(&normal);
            let ray = Ray {
                origin: position.clone(),
                direction: random_direction,
            };
            scene.cast_ray(&ray, ray_depth)
        });
        let colour = colours.sum::<Colour>() / self.secondary_rays as f32;

        colour * self.colour
    }
}

pub fn unit_vector_in_hemisphere(direction: &Vector3<f32>) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let vector = Vector3::<f32>::new(rng.gen(), rng.gen(), rng.gen()).map(|v| v * 2.0 - 1.0);
        let magnitude_sqr = vector.magnitude2();

        if magnitude_sqr > f32::MIN && magnitude_sqr < 1.0 {
            return (vector.normalize() + direction).normalize();
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckerMaterial {
    pub grid_size: f32,
}

#[typetag::serde]
impl Material for CheckerMaterial {
    fn get_colour(
        &self,
        _scene: &Scene,
        _view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        _normal: &Vector3<f32>,
        _ray_depth: u8,
    ) -> Colour {
        let value_x = position.x.abs() % (2.0 * self.grid_size) < self.grid_size;
        let value_y = position.y.abs() % (2.0 * self.grid_size) < self.grid_size;
        let value_z = position.z.abs() % (2.0 * self.grid_size) < self.grid_size;

        if value_x ^ value_y ^ value_z {
            colour::WHITE
        } else {
            colour::BLACK
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LightMaterial {
    pub colour: Colour,
}

#[typetag::serde]
impl Material for LightMaterial {
    fn get_colour(
        &self,
        _scene: &Scene,
        _view_direction: &Vector3<f32>,
        _position: &Point3<f32>,
        _normal: &Vector3<f32>,
        _ray_depth: u8,
    ) -> Colour {
        self.colour
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkyBoxMaterial {
    pub colour_bottom: Colour,
    pub colour_top: Colour,
}

#[typetag::serde]
impl Material for SkyBoxMaterial {
    fn get_colour(
        &self,
        _scene: &Scene,
        view_direction: &Vector3<f32>,
        _position: &Point3<f32>,
        _normal: &Vector3<f32>,
        _ray_depth: u8,
    ) -> Colour {
        Colour::lerp(
            self.colour_bottom,
            self.colour_top,
            0.5 + view_direction.normalize().y * 0.5,
        )
    }
}

#[test]
pub fn serialise_material() {
    let material_diffuse: &dyn Material = &DiffuseMaterial {
        colour: colour::LIGHT_GREY,
        secondary_rays: 64,
    };

    let _material_as_str = serde_json::to_string(&material_diffuse).unwrap();

    assert_eq!(_material_as_str, "{\"DiffuseMaterial\":{\"colour\":{\"r\":0.75,\"g\":0.75,\"b\":0.75,\"a\":1.0},\"secondary_rays\":64}}");
}

#[test]
pub fn deserialise_material() {
    let _material: Box<dyn Material> = serde_json::from_str(
        "{\"DiffuseMaterial\":{\"colour\":{\"r\":0.75,\"g\":0.75,\"b\":0.75,\"a\":1.0},\"secondary_rays\":64}}",
    )
    .unwrap();
}
