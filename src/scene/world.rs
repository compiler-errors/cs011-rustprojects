use shape::shape::Shape;
use shape::point_light::PointLight;

/// The World struct represents all of the objects in the scene that will be traced by the Camera.
pub struct World {
    objects: Vec<Box<Shape>>,
    lights: Vec<PointLight>
}

impl World {
    /// Adds a shape to the world.
    pub fn add_shape(&mut self, shape: Box<Shape>) {
        self.objects.push(shape);
    }

    /// Adds a PointLight to the world.
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
}
