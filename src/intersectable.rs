use crate::{hit::Hit, ray::Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

// #[derive()]
// pub struct Intersectables {
//     pub intersectables: Vec<Box<dyn Intersectable>>,
// }

// impl Intersectable for Intersectables {
//     fn intersect(&self, ray: &Ray) -> Option<Hit> {
//         let intersections = self.intersectables.iter().map(|i| i.intersect(ray));
//         let hit = intersections.fold(
//             Box::new(None),
//             |x: Box<Option<Hit>>, y: Box<Option<Hit>>| match *x {
//                 None => x,
//                 Some(x_hit) => match *y {
//                     None => y,
//                     Some(y_hit) => {
//                         if x_hit.distance < y_hit.distance {
//                             x
//                         } else {
//                             y
//                         }
//                     }
//                 },
//             },
//         );
//         *hit
//     }
// }
