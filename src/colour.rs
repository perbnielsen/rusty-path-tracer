use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Rem, Sub},
};

use cgmath::{VectorSpace, Zero};

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[allow(dead_code)]
pub const GREY: Colour = Colour {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};

#[allow(dead_code)]
pub const LIGHT_BLUE: Colour = Colour {
    r: 0.5,
    g: 0.7,
    b: 1.0,
    a: 1.0,
};

#[allow(dead_code)]
pub const RED: Colour = Colour {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

#[allow(dead_code)]
pub const GREEN: Colour = Colour {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};

#[allow(dead_code)]
pub const BLUE: Colour = Colour {
    r: 0.0,
    g: 0.0,
    b: 1.0,
    a: 1.0,
};

#[allow(dead_code)]
pub const WHITE: Colour = Colour {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

#[allow(dead_code)]
pub const BLACK: Colour = Colour {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

#[allow(dead_code)]
impl Mul<Self> for Colour {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Colour {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

#[allow(dead_code)]
impl Mul<f32> for Colour {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Colour {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a * other,
        }
    }
}

#[allow(dead_code)]
impl Div<f32> for Colour {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Colour {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
            a: self.a / other,
        }
    }
}

#[allow(dead_code)]
impl Add<Self> for Colour {
    type Output = Colour;
    fn add(self, other: Self) -> Self::Output {
        Colour {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

#[allow(dead_code)]
impl Sub<Self> for Colour {
    type Output = Colour;
    fn sub(self, other: Self) -> Self::Output {
        Colour {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

#[allow(dead_code)]
impl Rem<f32> for Colour {
    type Output = Colour;
    fn rem(self, other: f32) -> Self::Output {
        Colour {
            r: self.r % other,
            g: self.g % other,
            b: self.b % other,
            a: self.a % other,
        }
    }
}

impl Sum for Colour {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(BLACK, |sum, val| sum + val)
    }
}

impl Zero for Colour {
    fn zero() -> Self {
        BLACK.clone()
    }

    fn is_zero(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0 && self.a == 0.0
    }

    fn set_zero(&mut self) {
        self.r = BLACK.r;
        self.g = BLACK.g;
        self.b = BLACK.b;
        self.a = BLACK.a;
    }
}

impl VectorSpace for Colour {
    type Scalar = f32;

    fn lerp(self, other: Self, amount: Self::Scalar) -> Self {
        self + ((other - self) * amount)
    }
}
