use geom::Vec3;
use geom::Color;
use geom::Ray;
use shape::Light;
use shape::Intersection;
use scene::World;

/// PointLight represents a single, infinitely dense (1 dimensional) light source in the scene
pub struct PointLight {
    position: Vec3,
    color: Color
}

impl PointLight {
    pub fn new(position: Vec3, color: Color) -> PointLight {
        PointLight {position: position, color: color}
    }
}

impl Light for PointLight {
    fn in_shadow(&self, world: &World, intersection: &Intersection) -> Option<Vec3> {
        // We construct a ray from the point of intersection to the light. If it
        // intersects an object, we know there is no light getting to the object.
        // We use step_epsilon() so the Ray must not intersect from the object it is
        // being emitted from. Without it, we have a lot of black-dotted noise.
        let L = self.position - intersection.position;
        let shadow_ray = Ray::new(intersection.position, L).step_epsilon();

        if let Some(shadow) = world.get_closest_intersection(&shadow_ray) {
            if shadow.distance < L.mag() {
                return None;
            }
        }

        Some(L)
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
