use crate::ray::Ray;
use cgmath::{Basis3, Point3};

#[derive(Debug, Clone)]
pub struct Viewport {
    pub width: usize,
    pub height: usize,
    pub basis: Basis3<f32>,
    pub origin: Point3<f32>,
    pub fov: f32,
}

impl IntoIterator for Viewport {
    type Item = Ray;

    type IntoIter = ViewportIter;

    fn into_iter(self) -> Self::IntoIter {
        ViewportIter::make(self)
    }
}

pub struct ViewportIter {
    current_x: usize,
    current_y: usize,
    viewport: Viewport,
    fov_x: f32,
    fov_y: f32,
}

impl ViewportIter {
    pub fn make(viewport: Viewport) -> Self {
        let aspect_ratio = viewport.height as f32 / viewport.width as f32;
        let fov_x = (viewport.fov / 2.0).tan() * 2.0;
        let fov_y = fov_x * aspect_ratio;

        Self {
            current_x: 0,
            current_y: 0,
            viewport,
            fov_x,
            fov_y,
        }
    }
}

impl Iterator for ViewportIter {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.viewport.height {
            return None;
        }

        let x = self.fov_x * ((self.current_x as f32 / (self.viewport.width as f32 - 1.0)) - 0.5);
        let y = self.fov_y * ((self.current_y as f32 / (self.viewport.height as f32 - 1.0)) - 0.5);

        let basis = self.viewport.basis.as_ref();
        let direction = basis.z + (basis.x * x) + (y * basis.y);
        let next_ray = Ray::new(self.viewport.origin, direction);

        self.current_x += 1;
        if self.current_x >= self.viewport.width {
            self.current_x = 0;
            self.current_y += 1;
        }

        Some(next_ray)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use cgmath::{Basis3, InnerSpace, Rotation, Vector3};

    #[test]
    pub fn viewport_iterator_test() {
        let viewport = Viewport {
            width: 5,
            height: 5,
            basis: Basis3::look_at(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ),
            origin: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            fov: 90.0,
        };

        for i in viewport {
            assert_approx_eq!(i.direction.magnitude(), 1.0);
        }
    }
}
