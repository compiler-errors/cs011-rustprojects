use std::fmt;
use geom::vec3::Vec3;

/// Ray implements a physical ray in a 3d space for
/// the ray tracer. It consists of a point and a
/// direction vector eminating from the point.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl fmt::Display for Ray {
    // Formats a Ray struct for printing or stringifying.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray{{{} points {}}}", self.origin, self.direction)
    }
}

impl Ray {
    /// Constructs a ray from an origin, direction vector,
    /// a normalize boolean which normalizes the direction
    /// if true.
    pub fn new(origin: Vec3, direction: Vec3, normalize: bool) -> Ray {
        if normalize {
            Ray {origin: origin, direction: direction.norm()}
        } else {
            Ray {origin: origin, direction: direction}
        }
    }

    /// Constructs a ray from  origin and ending  poinst, and
    /// a normalize boolean which normalizes the direction
    /// if true.
    pub fn between(origin: Vec3, endpoint: Vec3, normalize: bool) -> Ray {
        if normalize {
            Ray {origin: origin, direction: (endpoint - origin).norm()}
        } else {
            Ray {origin: origin, direction: (endpoint - origin)}
        }
    }
}
