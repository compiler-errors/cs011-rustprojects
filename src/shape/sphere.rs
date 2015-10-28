use geom::Vec3;
use geom::Color;
use geom::Ray;
use shape::Shape;
use shape::Intersection;

pub struct Sphere {
    position: Vec3,
    radius: f64,
    color: Color
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, color: Color) -> Sphere {
        Sphere {position: position, radius: radius, color: color}
    }
}

impl Shape for Sphere {
    /// Returns the first, closest intersection of the Sphere.
    fn intersect_first(&self, ray: Ray) -> Option<Intersection> {
        // We'll define a bunch of names just to make our equations look prettier.
        let D = ray.direction;
        let P = ray.origin;
        let C = self.position;
        let r = self.radius;

        // Borrowed from the CS11 Adv. C++ Lab 3 page.
        let a = D * D;
        let b = 2.0 * (P * D - D * C);
        let c = P * P + C * C - 2.0 * (P * C) - r * r;

        let t1 = (-b - (b * b - 4.0 * a * c).sqrt())/(2.0 * a);
        if t1 > 0.0 {
            let distance = (D * D) * t1;
            let point = P + D * t1;
            let norm = (D - C).norm();
            return Some(Intersection::new(distance, self.color, point, norm));
        }

        let t2 = (-b + (b * b - 4.0 * a * c).sqrt())/(2.0 * a);
        if t2 > 0.0 {
            let distance = (D * D) * t2;
            let point = P + D * t2;
            let norm = (D - C).norm();
            return Some(Intersection::new(distance, self.color, point, norm));
        }

        None // :(
    }

    /// Returns a Vec which may contain at most two intersections.
    fn intersect_all(&self, ray: Ray) -> Vec<Intersection> {
        let mut ret = Vec::new();

        // We'll define a bunch of names just to make our equations look prettier.
        let D = ray.direction;
        let P = ray.origin;
        let C = self.position;
        let r = self.radius;

        // Borrowed from the CS11 Adv. C++ Lab 3 page.
        let a = D * D;
        let b = 2.0 * (P * D - D * C);
        let c = P * P + C * C - 2.0 * (P * C) - r * r;

        let t1 = (-b - (b * b - 4.0 * a * c).sqrt())/(2.0 * a);
        if t1 > 0.0 {
            let distance = (D * D) * t1;
            let point = P + D * t1;
            let norm = (D - C).norm();
            ret.push(Intersection::new(distance, self.color, point, norm));
        }

        let t2 = (-b + (b * b - 4.0 * a * c).sqrt())/(2.0 * a);
        if t2 > 0.0 {
            let distance = (D * D) * t2;
            let point = P + D * t2;
            let norm = (D - C).norm();
            ret.push(Intersection::new(distance, self.color, point, norm));
        }

        ret
    }

    /// Sets the position of the Sphere.
    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Gets a vector which describes the position of the Sphere.
    fn get_position(&self) -> Vec3 {
        self.position
    }

    /// Sets the color of the Sphere.
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Gets the color of the Shape.
    fn get_color(&self) -> Color {
        self.color
    }
}
