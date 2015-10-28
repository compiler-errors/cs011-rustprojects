use geom::vec3::Vec3;
use geom::color::Color;

/// The Intersection struct represents an intersection of a Ray and a Shape object for ray tracing.
#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f64,
    pub color: Color,
    pub point: Vec3,
    pub norm: Vec3
}

impl Intersection {
    pub fn new(distance: f64, color: Color, point: Vec3, norm: Vec3) -> Intersection {
        Intersection {distance: distance, color: color, point: point, norm: norm}
    }
}
