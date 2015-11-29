use std::ops::*;
use std::fmt;
use std::f64::consts::PI;
use ::rand::{Rng, thread_rng};

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

    /// Sample the hemisphere normal to the Vector
    ///
    /// This will return a vector that is normal to the plane that the Vector can be visualized
    /// to intersect fully. The parameter "pow" is the cosine-weighted power of the sample.
    /// A higher power will cause the distribution to take on a "cone shape" and be less likely
    /// to be close-to-orthogonal to the plane.
    pub fn sample_hemisphere(pow: f64) -> Vec3 {
        let x = jitter();
        let y = jitter();

        // From Suffern "Ray Tracing from the Ground Up" (7.3)
        let cos_phi = (PI * 2.0 * x).cos();
        let sin_phi = (PI * 2.0 * x).sin();
        let cos_theta = (1.0 - y).powf(1.0 / (pow + 1.0));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        Vec3::new(sin_theta * cos_phi, cos_theta, sin_theta * sin_phi)
    }

    // Some vector operations require an "up" vector which is not completely up in order
    // to protect against singularities that happen which an "up" vector lines up perfectly
    // with another vector.
    pub fn up() -> Vec3 {
        Vec3::new(0.00424, 1.0, 0.00764)
    }

    /// Calculates the vector's absolute magnitude.
    pub fn mag(&self) -> f64 {
        return (*self * *self).sqrt()
    }

    /// Returns a normalized copy of the vector.
    pub fn norm(&self) -> Vec3 {
        return *self / self.mag()
    }

    // Performs a vector projection operator "projecting" vector A
    // onto vector B.
    pub fn project(&self, b: Vec3) -> Vec3 {
        b * ((*self * b) / (b * b))
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

/// Returns a random real number on [0, 1]
///
/// I feel bad for copying code.
fn jitter() -> f64 {
    thread_rng().next_f64()
}
