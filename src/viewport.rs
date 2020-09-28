use crate::ray::Ray;
use cgmath::{Basis3, Point3};

#[derive(Debug, Clone)]
pub struct Viewport {
    width: usize,
    height: usize,
    basis: Basis3<f32>,
    origin: Point3<f32>,
    fov: f32,
    current_x: usize,
    current_y: usize,
    fov_x: f32,
    fov_y: f32,
}

impl Viewport {
    pub fn new(
        width: usize,
        height: usize,
        basis: Basis3<f32>,
        origin: Point3<f32>,
        fov: f32,

        current_x: usize,
        current_y: usize,
        fov_x: f32,
        fov_y: f32,
    ) -> Self {
        Self {
            width,
            height,
            basis,
            origin,
            fov,
            current_x,
            current_y,
            fov_x,
            fov_y,
        }
    }
}

impl Iterator for Viewport {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.height {
            return None;
        }

        let x = self.fov_x * ((self.current_x as f32 / (self.width as f32 - 1.0)) - 0.5);
        let y = self.fov_y * ((self.current_y as f32 / (self.height as f32 - 1.0)) - 0.5);

        let basis = self.basis.as_ref();
        let direction = basis.z + (basis.x * x) + (y * basis.y);
        let next_ray = Ray::new(self.origin, direction);

        self.current_x += 1;
        if self.current_x >= self.width {
            self.current_x = 0;
            self.current_y += 1;
        }

        Some(next_ray)
    }
}
