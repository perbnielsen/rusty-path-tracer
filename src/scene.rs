use cgmath::num_traits::identities::Zero;
use cgmath::EuclideanSpace;
use cgmath::{Point3, Vector3};

use crate::colour::{Colour, BLACK};
use crate::intersectable::Intersectable;
use crate::material::Material;
use crate::ray::Ray;

pub struct Scene {
    max_ray_depth: u8,
    root_intersectable: Box<dyn Intersectable>,
    background: Box<dyn Material>,
}

#[derive(Copy, Clone)]
pub struct SceneStatistics {
    pub total_number_of_rays_cast: u32,
    pub total_number_of_rays_killed: u32,
}

impl Scene {
    pub fn new(
        max_ray_depth: u8,
        root_intersectable: Box<dyn Intersectable>,
        background: Box<dyn Material>,
    ) -> Scene {
        Self {
            max_ray_depth,
            root_intersectable,
            background,
        }
    }

    pub fn cast_ray(&self, ray: &Ray, ray_depth: u8) -> Colour {
        {
            if ray_depth > self.max_ray_depth {
                return BLACK;
            }
        }

        let hit = self.root_intersectable.intersect(ray);
        match hit {
            Some(hit) => hit.material.get_colour(
                self,
                &ray.direction,
                &hit.position,
                &hit.normal,
                ray_depth + 1,
            ),
            None => self.background.get_colour(
                self,
                &ray.direction,
                &Point3::<f32>::origin(),
                &Vector3::<f32>::zero(),
                ray_depth + 1,
            ),
        }
    }
}
