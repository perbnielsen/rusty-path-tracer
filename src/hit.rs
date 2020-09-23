use crate::material::Material;
use cgmath::{Point3, Vector3};
use std::rc::Rc;

#[derive(Clone)]
pub struct Hit {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: Rc<dyn Material>,
}

impl Hit {
    pub fn new(
        distance: f32,
        position: Point3<f32>,
        normal: Vector3<f32>,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            distance,
            position,
            normal,
            material,
        }
    }
}
