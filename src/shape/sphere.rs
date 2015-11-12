use ::Rc;
use geom::Vec3;
use geom::Ray;
use shape::Shape;
use shape::Intersection;
use shape::Material;

pub struct Sphere {
    position: Vec3,
    radius: f64,
    material: Rc<Material>
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere {position: position, radius: radius, material: material}
    }

    /// Sets the position of the Sphere.
    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Gets a vector which describes the position of the Sphere.
    fn get_position(&self) -> Vec3 {
        self.position
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
        let b = 2.0 * (P * D - D * C);
        let c = P * P + C * C - 2.0 * (P * C) - r * r;

        let t1 = (-b - (b * b - 4.0 * c).sqrt()) / 2.0;
        if t1 > 0.0 {
            let point = P + D * t1;
            let norm = (point - C).norm();
            return Some(Intersection::new(t1, self.material.clone(), point, norm));
        }

        let t2 = (-b + (b * b - 4.0 * c).sqrt()) / 2.0;
        if t2 > 0.0 {
            let point = P + D * t2;
            let norm = (point - C).norm();
            return Some(Intersection::new(t2, self.material.clone(), point, norm));
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

        // Borrowed from the CS11 Adv. C++ Lab 3 page
        let b = 2.0 * (P * D - D * C);
        let c = P * P + C * C - 2.0 * (P * C) - r * r;

        let t1 = (-b - (b * b - 4.0 * c).sqrt()) / 2.0;
        if t1 > 0.0 {
            let distance = (D * D) * t1;
            let point = P + D * t1;
            let norm = (point - C).norm();
            ret.push(Intersection::new(distance, self.material.clone(), point, norm));
        }

        let t2 = (-b + (b * b - 4.0 * c).sqrt()) / 2.0;
        if t2 > 0.0 {
            let distance = (D * D) * t2;
            let point = P + D * t2;
            let norm = (point - C).norm();
            ret.push(Intersection::new(distance, self.material.clone(), point, norm));
        }

        ret
    }

    /// Sets the material of the Sphere.
    fn set_material(&mut self, material: Rc<Material>) {
        self.material = material;
    }

    /// Gets the material of the Sphere.
    fn get_material(&self) -> Rc<Material> {
        self.material.clone()
    }
}
