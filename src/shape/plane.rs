use geom::Vec3;
use geom::Color;
use geom::Ray;
use shape::Shape;
use shape::Intersection;

pub struct Plane {
    normal: Vec3,
    distance: f64,
    color: Color
}

impl Plane {
    pub fn new(normal: Vec3, distance: f64, color: Color) -> Plane {
        Plane {normal: normal, distance: distance, color: color}
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
            Some(Intersection::new((D * D) * t, self.color, P + D * t, N))
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

    /// Sets the color of the Shape.
    /// (for now, Shapes are of uniform color.)
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Gets the color of the Shape.
    fn get_color(&self) -> Color {
        self.color
    }
}
