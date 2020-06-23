use cgmath::{Point3, Vector3};

#[derive(Debug)]
pub struct Hit {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
    // pub material: Box<dyn Material>,
}

impl Hit {
    pub fn new(distance: f32, position: Point3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            distance,
            position,
            normal,
            // material,
        }
    }
}
