use geom::vec3::Vec3;
use geom::color::Color;

/// PointLight represents a single, infinitely dense (1 dimensional) light source in the scene
pub struct PointLight {
    pub position: Vec3,
    pub color: Color
}

impl PointLight {
    pub fn new(position: Vec3, color: Color) -> PointLight {
        PointLight {position: position, color: color}
    }
}
