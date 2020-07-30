use std::ops::Mul;

#[derive(Debug)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Mul<f32> for &Colour {
    type Output = Colour;
    fn mul(self, scalar: f32) -> Self::Output {
        Colour {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a,
        }
    }
}
