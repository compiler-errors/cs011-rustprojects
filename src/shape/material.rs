use geom::Color;

pub struct Material {
    pub matte_intensity: f64,
    pub matte_color: Color,
    pub glossy_intensity: f64,
    pub glossy_power: f64,
    pub glossy_color: Color
}

impl Material {
    pub fn new(matte_intensity: f64, matte_color: Color, glossy_intensity: f64,
               glossy_power: f64, glossy_color: Color) -> Material {
        Material {matte_intensity: matte_intensity, matte_color: matte_color,
            glossy_intensity: glossy_intensity, glossy_power: glossy_power,
            glossy_color: glossy_color}
    }

    // Returns a new "matte" material (100% matte_intensity).
    pub fn lambertian(color: Color) -> Material {
        Material::new(1.0, color, 0.0, 0.0, Color::black())
    }

    // Returns a new reflective color
    pub fn reflective(color: Color) -> Material {
        Material::new(0.1, Color::black(), 0.9, -1.0, color)
    }

    pub fn glossy(color: Color, glossy_power: f64) -> Material {
        Material::new(0.1, color, 0.9, glossy_power, color)
    }
}
