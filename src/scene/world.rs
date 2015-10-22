use shape::shape::Shape;
use shape::point_light::PointLight;

pub struct World {
    objects: Vec<Box<Shape>>,
    lights: Vec<PointLight>,
}

impl World {
    fn addShape(&mut self, shape: Box<Shape>) {
        self.objects.push(shape);
    }

    fn addPoint(&mut self, light: PointLight) {
        self.lights.push(light);
    }
}
