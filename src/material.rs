use crate::colour::Colour;
use crate::colour::{BLACK, WHITE};
use crate::ray::Ray;
use crate::scene::Scene;
use cgmath::InnerSpace;
use cgmath::Point3;
use cgmath::Vector3;
use rand::Rng;

pub trait Material {
    fn get_colour(
        &self,
        scene: &mut Scene,
        view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        normal: &Vector3<f32>,
        ray_depth: u8,
    ) -> Colour;
}

pub struct MirrorMaterial {
    pub colour: Colour,
}

impl Material for MirrorMaterial {
    fn get_colour(
        &self,
        scene: &mut Scene,
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

        scene.get_colour(&ray, ray_depth) * self.colour
    }
}

pub struct DiffuseMaterial {
    pub colour: Colour,
    pub secondary_rays: i32,
}

impl Material for DiffuseMaterial {
    fn get_colour(
        &self,
        scene: &mut Scene,
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
            scene.get_colour(&ray, ray_depth)
        });
        let colour = colours.sum::<Colour>() / self.secondary_rays as f32;

        colour * self.colour
    }
}

pub fn unit_vector_in_hemisphere(direction: &Vector3<f32>) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let vector = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            .map(|val| val * 2.0 - 1.0);

        let magnitude_sqr = vector.magnitude2();
        if magnitude_sqr > 0.001 && magnitude_sqr < 1.0 {
            return (vector.normalize() + direction).normalize();
        }
    }
}

pub struct CheckerMaterial {
    pub grid_size: f32,
}

impl Material for CheckerMaterial {
    fn get_colour(
        &self,
        _scene: &mut Scene,
        _view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        _normal: &Vector3<f32>,
        _ray_depth: u8,
    ) -> Colour {
        let value_x = position.x.abs() % (2.0 * self.grid_size) < self.grid_size;
        let value_y = position.y.abs() % (2.0 * self.grid_size) < self.grid_size;
        let value_z = position.z.abs() % (2.0 * self.grid_size) < self.grid_size;

        if value_x ^ value_y ^ value_z {
            WHITE
        } else {
            BLACK
        }
    }
}

pub struct LightMaterial {
    pub colour: Colour,
}

impl Material for LightMaterial {
    fn get_colour(
        &self,
        _scene: &mut Scene,
        _view_direction: &Vector3<f32>,
        _position: &Point3<f32>,
        _normal: &Vector3<f32>,
        _ray_depth: u8,
    ) -> Colour {
        self.colour
    }
}
