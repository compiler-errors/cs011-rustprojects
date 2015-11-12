use std::f64::INFINITY;
use geom::Color;
use geom::Ray;
use geom::Vec3;
use shape::Intersection;
use shape::Shape;
use shape::Light;

const MAX_ITER: i32 = 4;
const AMBIENT_SAMPLES: i32 = 9;

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
                let mut final_color = Color::black();

                // We add the contributing color of each light
                for light in self.lights.iter() {
                    if let Some(light_direction) = light.in_shadow(self, &intersection) {
                        let illum_color = self.light_color(&intersection, ray, &light_direction);
                        final_color = final_color + light.get_color() * illum_color;
                    }
                }

                final_color + self.refl_color(&intersection, ray, depth)
                            + self.bg_color(&intersection, ray, depth)
            }
        }
    }

    /// Returns the color due to direct illumination of an object
    pub fn light_color(&self, intersection: &Intersection, ray: &Ray, light_direction: &Vec3)
                       -> Color {
        let ref material = *(intersection.material);

        let cos = (light_direction.norm() * intersection.norm).max(0.0);
        let matte_illum = material.matte_color * material.matte_intensity * cos;

        let Dr = ray.direction + (-ray.direction).project(intersection.norm) * 2.0;
        //TODO: glossy angle
        let spec_illum = material.glossy_color * (Dr * -ray.direction).max(0.0).powi(100) *
                         material.glossy_intensity;

        matte_illum + spec_illum
    }

    pub fn refl_color(&self, intersection: &Intersection, ray: &Ray, depth: i32) -> Color {
        let ref material = *(intersection.material);

        if material.glossy_intensity == 0.0 {
            return Color::black();
        }

        let perfect_reflect = (ray.direction + (-ray.direction).project(intersection.norm) * 2.0).norm();

        if material.glossy_power == -1.0 {
            let reflected_ray = Ray::new(intersection.position, perfect_reflect).step_epsilon();

            material.glossy_color * self.trace_ray(&reflected_ray, depth+1)
                                  * material.glossy_intensity
        } else {
            let w = (perfect_reflect).norm(); //up
            let u = (Vec3::up() % w).norm();
            let v = u % w;

            let sample = Vec3::sample_hemisphere(material.glossy_power);
            let Dr = u * sample.x + w * sample.y + v * sample.z;

            let reflected_direction = (if Dr * intersection.norm < 0.0 {
                u * -sample.x + w * sample.y + v * -sample.z
            } else {
                Dr
            }).norm();

            let reflected_ray = Ray::new(intersection.position,
                                         reflected_direction).step_epsilon();

            material.glossy_color * self.trace_ray(&reflected_ray, depth+1)
                                  * material.glossy_intensity
        }
    }

    pub fn bg_color(&self, intersection: &Intersection, ray: &Ray, depth: i32) -> Color {
        let ref material = *(intersection.material);
        let mut color = Color::black();

        if AMBIENT_SAMPLES == 0 {
            return color;
        }

        for _ in 0..AMBIENT_SAMPLES {
            let sample = Vec3::sample_hemisphere(0.0);
            let w = intersection.norm; //up
            let v = (w % Vec3::up()).norm();
            let u = v % w;

            let shadow_direction = u * sample.x + w * sample.y + v * sample.z;
            let shadow_ray = Ray::new(intersection.position, shadow_direction).step_epsilon();
            color = color +
            if let Some(_) = self.get_closest_intersection(&shadow_ray) {
                Color::black() // I think if you make this a call to trace_ray then it becomes
                // global illumination.
            } else {
                let cos = (shadow_direction.norm() * intersection.norm).max(0.0);
                self.bg_color * material.matte_color * cos
            };
        }

        color * (1.0 / (AMBIENT_SAMPLES as f64))
    }
}
