use geom::*;
use shape::Intersection;
use shape::Light;
use scene::World;

pub struct DirectionLight {
    direction: Vec3,
    color: Color
}

impl DirectionLight {
    pub fn new(direction: Vec3, color: Color) -> DirectionLight {
        DirectionLight {direction: direction.norm(), color: color}
    }
}

impl Light for DirectionLight {
    fn in_shadow(&self, world: &World, intersection: &Intersection) -> Option<Vec3> {
        let shadow_ray = Ray::new(intersection.position, -(self.direction)).step_epsilon();

        if let Some(_) = world.get_closest_intersection(&shadow_ray) {
            return None;
        }

        Some(-(self.direction))
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
