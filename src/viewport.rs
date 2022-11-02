use crate::ray::Ray;
use cgmath::{Basis3, Matrix3, Point3};

#[derive(Debug, Clone)]
pub struct Viewport {
    width: f32,
    height: f32,
    basis: Matrix3<f32>,
    origin: Point3<f32>,
    current_x: f32,
    current_y: f32,
}

impl Viewport {
    pub fn new(
        width: usize,
        height: usize,
        basis: Basis3<f32>,
        origin: Point3<f32>,
        fov: f32,
    ) -> Self {
        let aspect_ratio = height as f32 / width as f32;
        let delta_x = (fov / 2.0).tan() * 2.0;
        let delta_y = delta_x * aspect_ratio;
        let mut basis = basis.as_ref().clone();
        basis.x = basis.x * delta_x;
        basis.y = basis.y * delta_y;

        Self {
            width: width as f32,
            height: height as f32,
            basis,
            origin,
            current_x: 0.0,
            current_y: 0.0,
        }
    }
}

impl Iterator for Viewport {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.height {
            return None;
        }

        let x = (self.current_x / self.width) - 0.5;
        let y = (self.current_y / self.height) - 0.5;

        let direction = self.basis.z + (x * self.basis.x) + (y * self.basis.y);
        let next_ray = Ray::new(self.origin, direction);

        self.current_x += 1.0;

        if self.current_x >= self.width {
            self.current_x = 0.0;
            self.current_y += 1.0;
        }

        Some(next_ray)
    }
}
