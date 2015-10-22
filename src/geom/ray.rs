use std::fmt;
use geom::vec3::Vec3;

/*
 * Ray implements a physical ray in a 3d space for
 * the ray tracer. It consists of a point and a
 * direction vector eminating from the point.
 *
 * Ray::between(origin, endpoint): Returns a ray that
 * begins at origin and ends at endpoint.
 * Ray::new(origin, direction): Returns a ray that
 * begins at origin and points in direction.
 */
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

/*
 * fmt() implements the string formatter for the Ray object
 * which outputs the ray as Ray{(x,y,z) points (x,y,z)}.
 */
impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray{{{} points {}}}", self.origin, self.direction)
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, normalize: bool) -> Ray {
        if normalize {
            Ray {origin: origin, direction: direction.norm()}
        } else {
            Ray {origin: origin, direction: direction}
        }
    }

    pub fn between(origin: Vec3, endpoint: Vec3, normalize: bool) -> Ray {
        if normalize {
            Ray {origin: origin, direction: (endpoint - origin).norm()}
        } else {
            Ray {origin: origin, direction: (endpoint - origin)}
        }
    }
}
