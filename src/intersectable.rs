use std::{cmp::Ordering, rc::Rc};

use cgmath::{InnerSpace, Point3, Vector3};

use crate::{hit::Hit, material::Material, ray::Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Intersectables {
    pub intersectables: Vec<Box<dyn Intersectable>>,
}

impl Intersectable for Intersectables {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.intersectables
            .iter()
            .filter_map(|i| i.intersect(ray))
            .into_iter()
            .min_by(|x, y| {
                x.distance
                    .partial_cmp(&y.distance)
                    .unwrap_or(Ordering::Equal)
            })
    }
}

pub struct Triangle {
    a: Point3<f32>,
    b: Point3<f32>,
    c: Point3<f32>,
    normal: Vector3<f32>,
    material: Rc<dyn Material>,
}

impl Triangle {
    pub fn new(a: Point3<f32>, b: Point3<f32>, c: Point3<f32>, material: Rc<dyn Material>) -> Self {
        let u = b - a;
        let v = c - a;
        Self {
            a,
            b,
            c,
            normal: u.cross(v),
            material,
        }
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        if self.normal.magnitude2() == 0.0 {
            // Triangle is degenerate
            return None;
        }

        let angle_from_normal_to_ray_direction = cgmath::dot(self.normal, ray.direction);
        if angle_from_normal_to_ray_direction.abs() < std::f32::EPSILON {
            // Ray is parallel to triangle
            return None;
        }

        let angle_from_normal_to_ray_origin = -cgmath::dot(self.normal, ray.origin - self.a);
        let ray_distance = angle_from_normal_to_ray_origin / angle_from_normal_to_ray_direction;
        if ray_distance < 0.0 {
            // Ray points away from triangle
            return None;
        }

        let intersection_point = ray.origin + ray_distance * ray.direction;

        // Check if the intersection is inside the triangle
        let u = self.b - self.a;
        let v = self.c - self.a;
        let uu = cgmath::dot(u, u);
        let uv = cgmath::dot(u, v);
        let vv = cgmath::dot(v, v);
        let w = intersection_point - self.a;
        let wu = cgmath::dot(w, u);
        let wv = cgmath::dot(w, v);
        let denominator = uv * uv - uu * vv;
        let s = (uv * wv - vv * wu) / denominator;
        let t = (uv * wu - uu * wv) / denominator;
        if s < 0.0 || s > 1.0 || t < 0.0 || (s + t) > 1.0 {
            return None;
        }

        Some(Hit {
            distance: ray_distance,
            position: intersection_point,
            normal: self.normal,
            material: self.material.clone(),
        })
    }
}
