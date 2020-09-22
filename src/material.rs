use crate::colour::Colour;
use crate::scene::Scene;
use cgmath::InnerSpace;
use cgmath::Vector3;

pub trait Material {
    fn get_colour(
        &self,
        scene: &Scene,
        view_direction: &Vector3<f32>,
        normal: &Vector3<f32>,
    ) -> Colour;
}

impl Material for SimpleMaterial {
    fn get_colour(
        &self,
        scene: &Scene,
        view_direction: &Vector3<f32>,
        normal: &Vector3<f32>,
    ) -> Colour {
        let normal_to_view = cgmath::dot(view_direction.normalize(), *normal).abs();
        &self.colour * normal_to_view
    }
}

pub struct SimpleMaterial {
    pub colour: Colour,
}
