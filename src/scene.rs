use std::{cell::RefCell, rc::Rc};

use cgmath::num_traits::identities::Zero;
use cgmath::EuclideanSpace;
use cgmath::{Point3, Vector3};

use crate::{colour, colour::Colour, intersectable::Intersectable, material::Material, ray::Ray};

pub struct Scene {
    pub statistics: RefCell<SceneStatistics>,
    max_ray_depth: u8,
    root_intersectable: Box<dyn Intersectable>,
    background: Rc<dyn Material>,
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
        background: Rc<dyn Material>,
    ) -> Scene {
        let statistics = RefCell::new(SceneStatistics {
            total_number_of_rays_cast: 0,
            total_number_of_rays_killed: 0,
        });

        Self {
            max_ray_depth,
            root_intersectable,
            statistics,
            background,
        }
    }

    pub fn cast_ray(&self, ray: &Ray, ray_depth: u8) -> Colour {
        {
            let mut statistics = self.statistics.borrow_mut();
            if ray_depth > self.max_ray_depth {
                statistics.total_number_of_rays_killed += 1;
                return colour::BLACK;
            }

            statistics.total_number_of_rays_cast += 1;
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
