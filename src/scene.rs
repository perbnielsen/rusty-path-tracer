use cgmath::{InnerSpace, Vector3, VectorSpace};

use crate::{
    colour::Colour, colour::BLACK, colour::LIGHT_BLUE, colour::WHITE, intersectable::Intersectable,
    ray::Ray,
};

pub struct Scene {
    pub max_ray_depth: u8,
    pub root_intersectable: Box<dyn Intersectable>,
    pub total_number_of_rays_checked: u32,
    pub total_number_of_rays_killed: u32,
}

impl Scene {
    pub fn get_colour(&mut self, ray: &Ray, ray_depth: u8) -> Colour {
        if ray_depth == 0 {
            self.total_number_of_rays_killed += 1;
            return BLACK;
        }

        self.total_number_of_rays_checked += 1;

        let hit = self.root_intersectable.intersect(ray);
        match hit {
            Some(hit) => hit.material.get_colour(
                self,
                &ray.direction,
                &hit.position,
                &hit.normal,
                ray_depth - 1,
            ),
            None => get_sky_colour(&ray.direction),
        }
    }
}

fn get_sky_colour(direction: &Vector3<f32>) -> Colour {
    Colour::lerp(LIGHT_BLUE, WHITE, 0.5 + direction.normalize().y * 0.5)
}
