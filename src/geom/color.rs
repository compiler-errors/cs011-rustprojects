use std::ops::*;
use std::fmt;

/*
 * Color is a struct which represents the colors that
 * are put onto images in Lux. There are a few
 * operators implemented on Color, as well as a few
 * other methods.
 * Colors are represented as tuples of f32 (floats).
 * It is unnecessary to represent colors with f64,
 * because there is not much more color depth that
 * we can recognize past the precision of f32.
 *
 * color * f32 = (color) scale color by f32 factor
 * color + color = (color) add two colors together
 * color - color = (color) subtract two colors
 *
 * Color::new(r, g, b) constructs a color.
 * color.clamp() = (color) returns a color which
 * has all of its components with a ceiling of 1.
 */
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {r: self.r + other.r,
               g: self.g + other.g,
               b: self.b + other.b}
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {r: self.r - other.r,
               g: self.g - other.g,
               b: self.b - other.b}
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {r: self.r * other.r,
               g: self.g * other.g,
               b: self.b * other.b}
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {r: self.r * other,
               g: self.g * other,
               b: self.b * other}
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {r: r, g: g, b: b}
    }

    pub fn black() -> Color {
        Color {r: 0.0, g: 0.0, b: 0.0}
    }

    /*
     * Clamp will return a copy of a color which has
     * its components set to 1 if exceeding 1.
     * (colors with a component greater than 1 does
     * not make sense in our natural color space.)
     */
    pub fn clamp(self) -> Color {
        Color {r: if self.r <= 1.0 {self.r} else {1.0},
               g: if self.r <= 1.0 {self.g} else {1.0},
               b: if self.r <= 1.0 {self.b} else {1.0}}
    }
}
