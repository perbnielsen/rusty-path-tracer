use crate::{hit::Hit, intersectable::Intersectable, ray::Ray};
use cgmath::{Point3, Vector3};

#[derive(Debug)]
pub struct Sphere {
    pub centre: Point3<f32>,
    pub radius: f32,
    // material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Point3<f32>, radius: f32) -> Self {
        Self { centre, radius }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let m = vector_from_to(self.centre, ray.origin);
        let b = cgmath::dot(m, ray.direction);
        let c = cgmath::dot(m, m) - self.radius * self.radius;

        if c > 0.0 && b > 0.0 {
            return None;
        }

        let discriminant = (b * b) - c;
        if discriminant < 0.0 {
            return None;
        }

        let t = -b - discriminant.sqrt();
        if t < 0.0 {
            return None;
        }

        let intersection_point = ray.origin + (t * ray.direction);

        Some(Hit::new(
            t,
            intersection_point,
            vector_from_to(self.centre, intersection_point),
            // self.material.clone(),
        ))
    }
}

fn vector_from_to(p1: Point3<f32>, p2: Point3<f32>) -> Vector3<f32> {
    Vector3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use matches::assert_matches;

    #[test]
    pub fn test_sphere_intersection() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, -6.0), Vector3::new(0.0, 0.0, 1.0));
        let hit = sphere.intersect(&ray);

        assert_matches!(hit, Some(_));

        let hit = hit.unwrap();

        assert_approx_eq!(hit.distance, 1.0);
    }
}
