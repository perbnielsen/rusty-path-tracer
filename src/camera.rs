use crate::viewport::Viewport;
use cgmath::{Basis3, Point3, Rotation, Vector3};

#[derive(Clone, Copy)]
pub struct Camera {
    basis: Basis3<f32>,
    fov: f32,
    origin: Point3<f32>,
}

impl Camera {
    pub fn new(origin: Point3<f32>, forward: Vector3<f32>, up: Vector3<f32>, fov: f32) -> Camera {
        let basis = Basis3::look_at(forward, up);
        Camera { basis, origin, fov }
    }

    pub fn left(&self) -> Vector3<f32> {
        self.basis.as_ref().x.clone()
    }

    pub fn up(&self) -> Vector3<f32> {
        self.basis.as_ref().y.clone()
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.basis.as_ref().z.clone()
    }

    pub fn translate(&mut self, vector: Vector3<f32>) {
        self.origin -= vector;
    }

    pub fn get_viewport(&self, width: usize, height: usize) -> Viewport {
        Viewport::new(width, height, self.basis, self.origin, self.fov)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let origin = Point3::new(0.0, 0.0, 5.0);
        let forward = Vector3::new(0.0, 0.0, -1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let fov = core::f32::consts::PI * 0.5;

        Camera::new(origin, forward, up, fov)
    }
}
