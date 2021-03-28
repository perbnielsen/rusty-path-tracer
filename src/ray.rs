use cgmath::{InnerSpace, Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        let direction = direction.normalize();
        Self { origin, direction }
    }
}
