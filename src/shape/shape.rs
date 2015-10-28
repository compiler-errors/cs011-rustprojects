use shape::intersection::Intersection;
use geom::vec3::Vec3;
use geom::color::Color;
use geom::ray::Ray;

/// Shape is the trait which all geometric shapes which interact in the scene are expected to
/// implement.

/// The methods within the trait have to do mostly with intersection calculation, but also define
/// the Shape's color and position too.
pub trait Shape {
    /// Returns the first, closest intersection of a Ray and the Shape, or None if there are no
    /// intersections.
    fn intersect_first(&self, ray: Ray) -> Option<Intersection>;

    /// Returns a Vec containing all possible intersections of a Ray and a Shape.
    fn intersect_all(&self, ray: Ray) -> Vec<Intersection>;

    /// Sets the position of the Shape.
    fn set_position(&mut self, position: Vec3);

    /// Gets a vector which describes the position of the Shape.
    fn get_position(&self) -> Vec3;

    /// Sets the color of the Shape.
    /// (for now, Shapes are of uniform color.)
    fn set_color(&mut self, color: Color);

    /// Gets the color of the Shape.
    fn get_color(&self) -> Color;
}
