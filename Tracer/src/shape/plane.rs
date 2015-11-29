use ::Arc;
use geom::Vec3;
use geom::Color;
use geom::Ray;
use shape::Shape;
use shape::Intersection;
use shape::Material;

pub struct Plane {
    normal: Vec3,
    distance: f64,
    material: Arc<Material>
}

impl Plane {
    pub fn new(normal: Vec3, distance: f64, material: Arc<Material>) -> Plane {
        Plane {normal: normal, distance: distance, material: material}
    }
}

impl Shape for Plane {
    /// Returns the first, closest intersection of a Ray and the Shape, or None if there are no
    /// intersections.
    fn intersect_first(&self, ray: Ray) -> Option<Intersection> {
        let D = ray.direction;
        let P = ray.origin;
        let N = self.normal;

        let det = D * N;

        if det == 0.0 {
            return None;
        }

        let t = -(P * N + self.distance) / det;

        if t >= 0.0 {
            Some(Intersection::new(t, self.material.clone(), P + D * t, N))
        } else {
            None
        }
    }

    /// Returns a Vec containing all possible intersections of a Ray and a Shape.
    fn intersect_all(&self, ray: Ray) -> Vec<Intersection> {
        match self.intersect_first(ray) {
            None => Vec::<Intersection>::new(),
            Some(intersection) => vec![intersection]
        }
    }

    /// Sets the material of the Plane.
    fn set_material(&mut self, material: Arc<Material>) {
        self.material = material;
    }

    /// Gets the material of the Plane.
    fn get_material(&self) -> Arc<Material> {
        self.material.clone()
    }
}
