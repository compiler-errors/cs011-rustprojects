use geom::Vec3;
use geom::Color;
use shape::Intersection;
use scene::World;

pub trait Light : Send + Sync {
    fn in_shadow(&self, world: &World, intersection: &Intersection) -> Option<Vec3>;
    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);
}
