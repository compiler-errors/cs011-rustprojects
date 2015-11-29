use ::Arc;
use geom::Vec3;
use shape::Material;

/// The Intersection struct represents an intersection of a Ray and a Shape object for ray tracing.
#[derive(Clone)]
pub struct Intersection {
    pub distance: f64,
    pub material: Arc<Material>,
    pub position: Vec3,
    pub norm: Vec3
}

impl Intersection {
    pub fn new(distance: f64, material: Arc<Material>, position: Vec3, norm: Vec3) -> Intersection {
        Intersection {distance: distance, material: material, position: position, norm: norm}
    }
}
