use std::rc::Rc;

use cgmath::{InnerSpace, Vector3};

use crate::{
    colour::lerp, colour::Colour, colour::LIGHT_BLUE, colour::WHITE, intersectable::Intersectable,
    ray::Ray,
};

pub struct Scene {
    pub root_intersectable: Rc<dyn Intersectable>,
}

impl Scene {
    pub fn get_colour(&self, ray: &Ray) -> Colour {
        let hit = self.root_intersectable.intersect(ray);
        match hit {
            Some(hit) => hit.material.get_colour(&self, &ray.direction, &hit.normal),
            None => get_sky_colour(&ray.direction),
        }
    }
}

fn get_sky_colour(direction: &Vector3<f32>) -> Colour {
    lerp(LIGHT_BLUE, WHITE, 0.5 + direction.normalize().y * 0.5)
}
