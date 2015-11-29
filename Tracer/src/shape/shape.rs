use ::Arc;
use shape::Intersection;
use shape::Material;
use geom::Ray;

/// Shape is the trait which all geometric shapes which interact in the scene are expected to
/// implement.

/// The methods within the trait have to do mostly with intersection calculation, but also define
/// the Shape's color and position too.
pub trait Shape : Send + Sync {
    /// Returns the first, closest intersection of a Ray and the Shape, or None if there are no
    /// intersections.
    fn intersect_first(&self, ray: Ray) -> Option<Intersection>;

    /// Returns a Vec containing all possible intersections of a Ray and a Shape.
    fn intersect_all(&self, ray: Ray) -> Vec<Intersection>;

    /// Sets the material of the Shape.
    fn set_material(&mut self, material: Arc<Material>);

    /// Gets the material of the Shape.
    fn get_material(&self) -> Arc<Material>;
}
