use std::f64::INFINITY;
use geom::Color;
use geom::Ray;
use geom::Vec3;
use shape::Intersection;
use shape::Shape;
use shape::Light;

/// The maximum recursive iterations that can be attained by the tracer.
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

    /// Traces a single ray and returns the intensity of light that is emitted through the ray.
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

    /// Returns the color due to direct illumination of an object by lights.
    pub fn light_color(&self, intersection: &Intersection, ray: &Ray, light_direction: &Vec3)
                       -> Color {
        // We can get a temporary (borrowed) reference to the material by the "ref" keyword
        let ref material = *(intersection.material);

        // We calculate the Lambertian BRDF by taking the cosine of the light and the normal
        let cos = (light_direction.norm() * intersection.norm).max(0.0);
        let matte_illum = material.matte_color * material.matte_intensity * cos;

        let Dr = ray.direction + (-ray.direction).project(intersection.norm) * 2.0;
        let spec_illum = material.glossy_color *
                         (Dr * -ray.direction).max(0.0).powf(material.glossy_power) *
                         material.glossy_intensity;

        matte_illum + spec_illum
    }

    /// Returns the color due to specular reflection from other objects.
    pub fn refl_color(&self, intersection: &Intersection, ray: &Ray, depth: i32) -> Color {
        // We can get a temporary (borrowed) reference to the material by the "ref" keyword
        let ref material = *(intersection.material);

        // If there is no glossy intensity, then quit early.
        if material.glossy_intensity == 0.0 {
            return Color::black();
        }

        // The ray that represents the perfect (mirror) reflection
        let perfect_reflect = (ray.direction + (-ray.direction).project(intersection.norm)
                               * 2.0).norm();

        // glossy_power == -1 represents a perfect mirror.
        if material.glossy_power == -1.0 {
            let reflected_ray = Ray::new(intersection.position, perfect_reflect).step_epsilon();

            material.glossy_color * self.trace_ray(&reflected_ray, depth+1)
                                  * material.glossy_intensity
        } else {
            // Construct a orthonormal coordinate system aligned upwards to the surface normal
            let w = (perfect_reflect).norm(); //up
            let u = (Vec3::up() % w).norm();
            let v = u % w;

            let sample = Vec3::sample_hemisphere(material.glossy_power);
            let Dr = u * sample.x + w * sample.y + v * sample.z;

            // This is a bit of a convoluted line, but it keeps me from having to use "mut"... We
            // check if the new emitted ray is "under the surface" of the object. If so, we rotate
            // it around its y axis 180 degrees and return the new outward facing vector.
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

    // Returns the color due to diffuse reflection from other objects
    pub fn bg_color(&self, intersection: &Intersection, ray: &Ray, depth: i32) -> Color {
        let ref material = *(intersection.material);
        let mut color = Color::black();

        // We can quit early if there's no matte_intensity
        if material.matte_intensity == 0.0 {
            return Color::black();
        }

        // Sample a hemisphere vector and construct a basis centered around the norm
        let sample = Vec3::sample_hemisphere(0.0);
        let w = intersection.norm; //up
        let v = (w % Vec3::up()).norm();
        let u = v % w;

        let shadow_direction = u * sample.x + w * sample.y + v * sample.z;
        let shadow_ray = Ray::new(intersection.position, shadow_direction).step_epsilon();

        let shadow_color = self.trace_ray(&shadow_ray, depth+1);
        // We can calculate a cosine of the normal and the shadow direction because the shadow
        // technically is emitted by a "light" which is just the other object emitting diffuse
        // light.
        let cos = (shadow_direction.norm() * intersection.norm).max(0.0);

        shadow_color * material.matte_color * material.matte_intensity * cos
    }
}
