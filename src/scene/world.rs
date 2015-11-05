use std::f64::INFINITY;
use geom::Color;
use geom::Ray;
use geom::Vec3;
use shape::Intersection;
use shape::Shape;
use shape::Light;

const MAX_ITER: i32 = 4;

/// The World struct represents all of the objects in the scene that will be traced by the Camera.
pub struct World {
    objects: Vec<Box<Shape>>,
    lights: Vec<Box<Light>>,
    bg_color: Color
}

impl World {
    /// Constructs an empty world.
    pub fn new(bg_color: Color) -> World {
        World {objects: Vec::new(), lights: Vec::new(), bg_color: bg_color}
    }

    /// Adds a shape to the world.
    pub fn add_shape(&mut self, shape: Box<Shape>) {
        self.objects.push(shape);
    }

    /// Adds a PointLight to the world.
    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    /// Returns the closest intersection to a ray.
    pub fn get_closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut distance = INFINITY;
        let mut closest: Option<Intersection> = None;

        for obj in self.objects.iter() {
            if let Some(intersection) = obj.intersect_first(*ray) {
                if intersection.distance < distance {
                    distance = intersection.distance;
                    closest = Some(intersection);
                }
            }
        }

        closest
    }

    pub fn trace_ray(&self, ray: &Ray, depth: i32) -> Color {
        if depth > MAX_ITER {
            return Color::black();
        }

        let closest = self.get_closest_intersection(ray);

        match closest {
            None => {
                //TODO: background color
                self.bg_color
            },
            Some(intersection) => {
                // For now, we don't want to use a BDRF...
                let mut final_color = self.bg_color;

                // We add the contributing color of each light
                for light in self.lights.iter() {
                    if let Some(light_direction) = light.in_shadow(self, &intersection) {
                        let material_color = self.color(&intersection, &light_direction, depth);
                        final_color = final_color
                                    + light.get_color() * material_color;
                    }
                }

                final_color
            }
        }
    }

    pub fn color(&self, intersection: &Intersection, light_direction: &Vec3, depth: i32) -> Color {
        let ref material = *(intersection.material);
        let cos = (light_direction.norm() * intersection.norm).max(0.0);
        material.matte_color * material.matte_intensity * cos
    }
}
