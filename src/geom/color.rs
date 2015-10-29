use std::ops::*;
use std::fmt;

/// Color stores the colors emitted by objects in Lux, represented by 3 tuple components (r, g, b).
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

/// Prepares the vector for printing or stringifying.
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

/// Two colors can be added by-components.
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {r: self.r + other.r,
               g: self.g + other.g,
               b: self.b + other.b}
    }
}

/// Two colors can be subtracted by-components.
impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {r: self.r - other.r,
               g: self.g - other.g,
               b: self.b - other.b}
    }
}

/// Two colors can be multiplied by-components.
impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {r: self.r * other.r,
               g: self.g * other.g,
               b: self.b * other.b}
    }
}

/// Multiplying a color and a scalar scales the color's components.
impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        //I need to cast my scaling factor to f32 because of the data type of color.
        let scale = other as f32;
        Color {r: self.r * scale,
               g: self.g * scale,
               b: self.b * scale}
    }
}

impl Color {
    /// Constructs a color from components.
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {r: r, g: g, b: b}
    }

    /// Constructs a plain black color struct.
    pub fn black() -> Color {
        Color {r: 0.0, g: 0.0, b: 0.0}
    }

    /// Clamp will return a copy of a color which has
    /// its components set to 1 if exceeding 1.
    ///
    /// Colors with a component greater than 1 do
    /// not make sense in our natural color space,
    /// so they need to be clamped before they are
    /// outputted to any image.
    pub fn clamp(self) -> Color {
        Color {r: if self.r <= 1.0 {self.r} else {1.0},
               g: if self.r <= 1.0 {self.g} else {1.0},
               b: if self.r <= 1.0 {self.b} else {1.0}}
    }
}
