use std::cmp::Ordering;

use crate::{hit::Hit, ray::Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Intersectables {
    pub intersectables: Vec<Box<dyn Intersectable>>,
}

impl Intersectable for Intersectables {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.intersectables
            .iter()
            .filter_map(|i| i.intersect(ray))
            .into_iter()
            .min_by(|x, y| {
                x.distance
                    .partial_cmp(&y.distance)
                    .unwrap_or(Ordering::Equal)
            })
    }
}
