use crate::{hit::Hit, material::Material, ray::Ray};
use cgmath::{InnerSpace, Point3};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, rc::Rc};

#[typetag::serde]
pub trait Intersectable: Debug {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Intersectables {
    pub intersectables: Vec<Box<dyn Intersectable>>,
}

#[typetag::serde]
impl Intersectable for Intersectables {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.intersectables
            .iter()
            .filter_map(|i| i.intersect(ray))
            .into_iter()
            .min_by(|x, y| x.distance.partial_cmp(&y.distance).unwrap())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Triangle {
    pub a: Point3<f32>,
    pub b: Point3<f32>,
    pub c: Point3<f32>,
    pub material: Rc<dyn Material>,
}

#[typetag::serde]
impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let u = self.b - self.a;
        let v = self.c - self.a;

        let normal = u.cross(v);

        if normal.magnitude2() == 0.0 {
            // Triangle is degenerate
            return None;
        }

        let angle_from_normal_to_ray_direction = cgmath::dot(normal, ray.direction);
        if angle_from_normal_to_ray_direction.abs() < std::f32::EPSILON {
            // Ray is parallel to triangle
            return None;
        }

        let angle_from_normal_to_ray_origin = -cgmath::dot(normal, ray.origin - self.a);
        let ray_distance = angle_from_normal_to_ray_origin / angle_from_normal_to_ray_direction;
        if ray_distance < 0.0 {
            // Ray points away from triangle
            return None;
        }

        let intersection_point = ray.origin + ray_distance * ray.direction;

        // Check if the intersection is inside the triangle
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
            normal,
            material: self.material.clone(),
        })
    }
}

#[test]
pub fn test_scene_loading() {
    let scene: Vec<Box<dyn crate::intersectable::Intersectable>> = serde_json::from_str(
        "[
            { \"Sphere\" : 
                {
                    \"centre\": {
                        \"x\": 0,
                        \"y\": 0,
                        \"z\": 0
                    },
                    \"radius\": 2.0,
                    \"material\": {
                        \"DiffuseMaterial\": {
                            \"colour\": {
                                \"r\": 0.75,
                                \"g\": 0.75,
                                \"b\": 0.75,
                                \"a\": 1.0
                            },
                            \"secondary_rays\": 64
                        }
                    }
                }
            }
        ]",
    )
    .unwrap();

    assert_ne!(0, scene.len());
}
