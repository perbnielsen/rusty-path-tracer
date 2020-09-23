use std::ops::Mul;

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

// impl Mul<f32> for &Colour {
//     type Output = Colour;
//     fn mul(self, scalar: f32) -> Self::Output {
//         Colour {
//             r: self.r * scalar,
//             g: self.g * scalar,
//             b: self.b * scalar,
//             a: self.a,
//         }
//     }
// }

impl Mul<Colour> for Colour {
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

// impl Add<Colour> for &Colour {
//     type Output = Colour;
//     fn add(self, other: Colour) -> Self::Output {
//         Colour {
//             r: self.r + other.r,
//             g: self.g + other.g,
//             b: self.b + other.b,
//             a: self.a + other.a,
//         }
//     }
// }

impl Colour {
    pub fn add(self, other: &Colour) -> Colour {
        Colour {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }

    pub fn mul(self, scalar: f32) -> Colour {
        Colour {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a,
        }
    }
}

// pub fn add(this: &Colour, other: &Colour) -> Colour {
//     Colour {
//         r: this.r + other.r,
//         g: this.g + other.g,
//         b: this.b + other.b,
//         a: this.a + other.a,
//     }
// }

// Implement Innerspace for Colour?

pub fn lerp(colour_a: Colour, colour_b: Colour, amount: f32) -> Colour {
    Colour {
        r: (1.0 - amount) * colour_a.r + amount * colour_b.r,
        g: (1.0 - amount) * colour_a.g + amount * colour_b.g,
        b: (1.0 - amount) * colour_a.b + amount * colour_b.b,
        a: (1.0 - amount) * colour_a.a + amount * colour_b.a,
    }
}
