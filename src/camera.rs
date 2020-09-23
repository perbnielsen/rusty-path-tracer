use crate::viewport::Viewport;
use cgmath::{Basis3, Point3, Rotation, Vector3};

pub struct Camera {
    pub basis: Basis3<f32>,
    pub fov: f32,
    pub origin: Point3<f32>,
}

impl Camera {
    pub fn new(origin: Point3<f32>, forward: Vector3<f32>, up: Vector3<f32>, fov: f32) -> Camera {
        // let origin = Point3::new(0.0, 0.0, 5.0);
        // let forward: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
        // let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        // let fov = core::f32::consts::PI * 0.5;
        let basis = Basis3::look_at(forward, up);

        Camera { basis, origin, fov }
    }

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
