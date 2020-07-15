use crate::viewport::Viewport;
use cgmath::{Basis3, Point3};

pub struct Camera {
    pub basis: Basis3<f32>,
    pub fov: f32,
    pub origin: Point3<f32>,
}

impl Camera {
    pub fn get_viewport(&self, width: usize, height: usize) -> Viewport {
        Viewport {
            width,
            height,
            basis: self.basis,
            origin: self.origin,
            fov: self.fov,
        }
    }
}
