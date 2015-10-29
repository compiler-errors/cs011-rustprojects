use std::ops::*;
use std::fmt;

/// Vec3 is the 3-dimensional vector struct that will handle
/// most of the geometry in Lux.
///
/// I have implemented most of the operations on the vector
/// to be handled with the regular language operators
/// (+ -/// ...).
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    /// Constructs a new vector from 3 components.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    /// Calculates the vector's absolute magnitude.
    pub fn mag(self) -> f64 {
        return (self * self).sqrt()
    }

    /// Returns a normalized copy of the vector.
    pub fn norm(self) -> Vec3 {
        return self / self.mag()
    }
}

/// Prepares the vector for printing or stringifying.
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Adds two vectors by-components and returns new vector.
impl Add for Vec3 {
    type Output = Vec3;
    //Not as concise as operator overloading in C++...

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x + other.x,
              y: self.y + other.y,
              z: self.z + other.z}
    }
}

/// Subtracts two vectors by-components and returns new vector.
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x - other.x,
              y: self.y - other.y,
              z: self.z - other.z}
    }
}

/// Returns dot product between two vectors.
impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Returns a scaled copy of the vector.
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scale: f64) -> Vec3 {
        Vec3 {x: self.x * scale,
              y: self.y * scale,
              z: self.z * scale}
    }
}

/// Returns a scaled copy of the vector.
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scale: f64) -> Vec3 {
        assert!(scale != 0.0, "Cannot divide vector components by 0!");

        Vec3 {x: self.x / scale,
              y: self.y / scale,
              z: self.z / scale}
    }
}

/// Returns the cross-product of two vectors.
impl Rem for Vec3 {
    type Output = Vec3;

    fn rem(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.y * other.z - self.z * other.y,
              y: self.z * other.x - self.x * other.z,
              z: self.x * other.y - self.y * other.x}
    }
}

/// Returns a copy of the vector with negated components.
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {x: -(self.x), y: -(self.y), z: -(self.z)}
    }
}
