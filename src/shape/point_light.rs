use geom::vec3::Vec3;
use geom::color::Color;

/// PointLight represents a single, infinitely dense (1 dimensional) light source in the scene
pub struct PointLight {
    origin: Vec3,
    color: Color
}
