use crate::colour::{Colour, BLACK};
use crate::ray::Ray;
use crate::scene::Scene;
use cgmath::InnerSpace;
use cgmath::Point3;
use cgmath::Vector3;
use rand::Rng;

pub trait Material {
    fn get_colour(
        &self,
        scene: &Scene,
        view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        normal: &Vector3<f32>,
    ) -> Colour;
}

impl Material for SimpleMaterial {
    fn get_colour(
        &self,
        scene: &Scene,
        _view_direction: &Vector3<f32>,
        position: &Point3<f32>,
        normal: &Vector3<f32>,
    ) -> Colour {
        // let normal_to_view = cgmath::dot(view_direction.normalize(), *normal).abs();

        let colours = (0..self.secondary_rays).map(|_| {
            let random_direction = unit_vector_in_hemisphere(&normal.normalize());
            let ray = Ray {
                origin: position.clone(),
                direction: random_direction,
            };
            scene.get_colour(&ray)
        });
        let _collected_colours: Vec<Colour> = colours.clone().collect();
        let colour = colours
            .fold(BLACK, |x, y| x.add(&y))
            .mul(1.0 / self.secondary_rays as f32);

        colour * self.colour

        // Colour {
        //     r: colour.r * self.colour.r,
        //     g: colour.g * self.colour.g,
        //     b: colour.b * self.colour.b,
        //     a: 1.0,
        // }

        // &self.colour * normal_to_view
    }
}

pub fn unit_vector_in_hemisphere(direction: &Vector3<f32>) -> Vector3<f32> {
    // direction.clone()
    let mut rng = rand::thread_rng();
    loop {
        let vector = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        let magnitude = vector.magnitude2();
        if magnitude > 0.1 && magnitude < 1.0 {
            // && cgmath::dot(*direction, vector) > 0.0 {
            return (vector.normalize() + direction).normalize();
        }
    }
}

pub struct SimpleMaterial {
    pub colour: Colour,
    pub secondary_rays: i32,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn unit_vector_in_hemisphere_test() {
        let vector = Vector3::new(1.0, 0.0, 0.0);
        let random_direction = unit_vector_in_hemisphere(&vector);

        assert_eq()
    }
}
