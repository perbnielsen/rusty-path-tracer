use crate::viewport::Viewport;
use cgmath::{Basis3, Point3, Rotation, Vector3};

pub struct Camera {
    pub basis: Basis3<f32>,
    pub fov: f32,
    pub origin: Point3<f32>,
}

impl Camera {
    pub fn new(origin: Point3<f32>, forward: Vector3<f32>, up: Vector3<f32>, fov: f32) -> Camera {
        let basis = Basis3::look_at(forward, up);
        Camera { basis, origin, fov }
    }

    pub fn get_viewport(&self, width: usize, height: usize) -> Viewport {
        let aspect_ratio = height as f32 / width as f32;
        let fov_x = (self.fov / 2.0).tan() * 2.0;
        let fov_y = fov_x * aspect_ratio;

        Viewport::new(
            width,
            height,
            self.basis,
            self.origin,
            self.fov,
            0,
            0,
            fov_x,
            fov_y,
        )
    }
}
