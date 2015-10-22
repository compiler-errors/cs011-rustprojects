use std::ops::*;
use std::fmt;

/*
 * Vec3 is the 3-dimensional vector struct that will handle
 * most of the geometry in Lux. I have implemented most of
 * the operations on the vector to be handled with the
 * regular language operators (+ - * ...).
 *
 * Operations:
 * vector + vector = (vector) simple vector sum
 * vector - vector = (vector) simple vector subtraction
 * vector * f64 = (vector) will scale vector by scalar amount
 * vector / f64 = (vector) will scale vector by scalar amount
 * vector * vector = (f64) dot product of two vectors
 * vector % vector = (vector) cross product of two vectors
 * -vector = (vector) negates the compontents of the vector
 *
 * In addition, the Vec3 struct has two class methods
 * Vector::new(x, y, z) makes a new vector
 * vector.norm() = (vector) normalized vector
 * vector.mag() = (f64) magnitude of vector
 *
 * Vec3 also implements the "fmt" trait. Therefore,
 * print!("{}", vector) will print the vector as a 3-tuple.
 *      -> "(x, y, z)"
 */
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn mag(self) -> f64 {
        return (self * self).sqrt()
    }

    pub fn norm(self) -> Vec3 {
        return self / self.mag()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    //Not as concise as operator overloading in C++...

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x + other.x,
              y: self.y + other.y,
              z: self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x - other.x,
              y: self.y - other.y,
              z: self.z - other.z}
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scale: f64) -> Vec3 {
        Vec3 {x: self.x * scale,
              y: self.y * scale,
              z: self.z * scale}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scale: f64) -> Vec3 {
        Vec3 {x: self.x / scale,
              y: self.y / scale,
              z: self.z / scale}
    }
}

impl Rem for Vec3 {
    type Output = Vec3;

    fn rem(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.y * other.z - self.z * other.y,
              y: self.z * other.x - self.x * other.z,
              z: self.x * other.y - self.y * other.x}
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {x: -(self.x), y: -(self.y), z: -(self.z)}
    }
}
