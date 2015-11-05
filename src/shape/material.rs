use geom::Color;

pub struct Material {
    pub matte_intensity: f64,
    pub matte_color: Color,
    pub glossy_intensity: f64,
    //glossy_power: f64,
    pub glossy_color: Color
}

impl Material {
    pub fn new(matte_intensity: f64, matte_color: Color, glossy_intensity: f64,
               glossy_color: Color) -> Material {
        Material {matte_intensity: matte_intensity, matte_color: matte_color,
                  glossy_intensity: glossy_intensity, glossy_color: glossy_color}
    }

    pub fn lambertian(color: Color) -> Material {
        Material::new(1.0, color, 0.0, Color::black())
    }
}
