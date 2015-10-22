use geom::vec3::Vec3;
use geom::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    distance: f64,
    color: Color,
    point: Vec3,
    norm: Vec3
}
