use std::f64::INFINITY;
use geom::Color;
use geom::Ray;
use geom::Vec3;
use shape::Intersection;
use shape::Shape;
use shape::PointLight;

/// The World struct represents all of the objects in the scene that will be traced by the Camera.
pub struct World {
    objects: Vec<Box<Shape>>,
    lights: Vec<Box<PointLight>>
}

impl World {
    /// Constructs an empty world.
    pub fn new() -> World {
        World {objects: Vec::new(), lights: Vec::new()}
    }

    /// Adds a shape to the world.
    pub fn add_shape(&mut self, shape: Box<Shape>) {
        self.objects.push(shape);
    }

    /// Adds a PointLight to the world.
    pub fn add_light(&mut self, light: Box<PointLight>) {
        self.lights.push(light);
    }

    /// Returns the closest intersection to a ray.
    fn get_closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut distance = INFINITY;
        let mut closest: Option<Intersection> = None;

        for obj in self.objects.iter() {
            if let Some(intersection) = obj.intersect_first(*ray) {
                if intersection.distance < distance {
                    closest = Some(intersection);
                    distance = intersection.distance;
                }
            }
        }

        closest
    }

    pub fn trace_ray(&self, ray: &Ray) -> Color {
        let closest = self.get_closest_intersection(ray);

        match closest {
            None => {
                //TODO: background color
                Color::black()
            },
            Some(intersection) => {
                // For now, we don't want to use a BDRF...
                 let mut final_color = Color::black();

                 // We add the contributing color of each light
                 for light in self.lights.iter() {
                     // We construct a ray from the point of intersection to the light. If it
                     // intersects an object, we know there is no light getting to the object.
                     // We use step_epsilon() so the Ray must not intersect from the object it is
                     // being emitted from. Without it, we have a lot of black-dotted noise.
                     let L = (light.position - intersection.point).norm();
                     let shadow_ray = Ray::new(intersection.point, L, false).step_epsilon();

                     // We use the "if let" matching syntax, but Some(_) tells the compiler that we
                     // don't actually care about the intersection itself.
                     if let Some(_) = self.get_closest_intersection(&shadow_ray) {
                         continue; //skip this light.
                     }

                     // Matte color BDRF
                     let intensity = (L * intersection.norm).max(0.0);
                     final_color = final_color + intersection.color * light.color * intensity;
                 }

                 final_color
            }
        }
    }
}
