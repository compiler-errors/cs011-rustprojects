use shape::intersection::Intersection;
use geom::vec3::Vec3;
use geom::color::Color;
use geom::ray::Ray;

/*
 * Shape is the trait which all geometric shapes which interact
 * in the scene are expected to implement. The methods within
 * the trait have to do mostly with intersection calculation,
 * but also define the Shape's color and position too.
 *
 * 
 */
pub trait Shape {
    fn intersect_first(&self, ray: Ray) -> Option<Intersection>;
    fn intersect_all(&self, ray: Ray) -> Option<Vec<Intersection>>;
    fn set_position(&mut self, position: Vec3);
    fn get_position(&self) -> Vec3;
    fn set_color(&mut self, color: Color);
    fn get_color(&self) -> Color;
}
