use std::ops::Mul;

#[derive(Debug)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// pub const GREY: Colour = Colour {
//     r: 0.5,
//     g: 0.5,
//     b: 0.5,
//     a: 1.0,
// };

pub const LIGHT_BLUE: Colour = Colour {
    r: 0.5,
    g: 0.7,
    b: 1.0,
    a: 1.0,
};

pub const BLUE: Colour = Colour {
    r: 0.2,
    g: 0.2,
    b: 1.0,
    a: 1.0,
};

pub const WHITE: Colour = Colour {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

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

// Implement Innerspace for Colour?

pub fn lerp(colour_a: Colour, colour_b: Colour, amount: f32) -> Colour {
    Colour {
        r: (1.0 - amount) * colour_a.r + amount * colour_b.r,
        g: (1.0 - amount) * colour_a.g + amount * colour_b.g,
        b: (1.0 - amount) * colour_a.b + amount * colour_b.b,
        a: (1.0 - amount) * colour_a.a + amount * colour_b.a,
    }
}
