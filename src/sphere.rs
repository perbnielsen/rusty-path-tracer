use crate::{hit::Hit, intersectable::Intersectable, ray::Ray, Material};
use cgmath::{InnerSpace, Point3};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sphere {
    pub centre: Point3<f32>,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

#[typetag::serde]
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let m = ray.origin - self.centre;
        let b = cgmath::dot(m, ray.direction);
        let c = cgmath::dot(m, m) - self.radius * self.radius;

        if c > 0.0 && b > 0.0 {
            return None;
        }

        let discriminant = (b * b) - c;
        if discriminant < 0.0 {
            return None;
        }

        let distance = -b - discriminant.sqrt();
        if distance < 0.0 {
            return None;
        }

        let intersection_point = ray.origin + (distance * ray.direction);
        let normal = (intersection_point - self.centre).normalize();

        Some(Hit::new(
            distance,
            intersection_point,
            normal,
            self.material.clone(),
        ))
    }
}
