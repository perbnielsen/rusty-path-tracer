use crate::{hit::Hit, ray::Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}
