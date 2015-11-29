use ::Arc;
use geom::Vec3;
use geom::Ray;
use shape::Shape;
use shape::Intersection;
use shape::Material;

pub struct Cylinder {
    start_cap: Vec3,
    axis: Vec3,
    radius: f64,
    height: f64,
    material: Arc<Material>
}

impl Cylinder {
    pub fn new(start_cap: Vec3, axis: Vec3, radius: f64, height: f64, material: Arc<Material>) ->
           Cylinder {
        Cylinder {start_cap: start_cap, axis: axis.norm(), radius: radius,
                  height: height, material: material}
    }
}

impl Shape for Cylinder {
    /// Returns the first, closest intersection of a Ray and the Shape, or None if there are no
    /// intersections.
    fn intersect_first(&self, ray: Ray) -> Option<Intersection> {
        let D = ray.direction;
        let V = self.axis;
        let X = ray.origin - self.start_cap;

        // Cylinder intersection points are solutions to at^2+bt+c=0.
        let a = 1.0 - (D * V).powi(2);
        let b = 2.0 * (D * X - (D * V) * (X * V));
        let c = X * X - (X * V).powi(2) - self.radius.powi(2);

        let t1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t1 > 0.0 {
            let m = (D * V) * t1 + X * V;

            if m > 0.0 && m < self.height {
                let point = ray.origin + D * t1;
                let norm_point = point - self.start_cap;
                let norm = (norm_point - norm_point.project(V)).norm();
                return Some(Intersection::new(t1, self.material.clone(), point, norm));
            }
        }

        let t2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t2 > 0.0 {
            let m = (D * V) * t2 + X * V;

            if m > 0.0 && m < self.height {
                let point = ray.origin + D * t2;
                // we define a factor to flip the normal if we are looking in the cylinder from
                // within the cap hole. Otherwise, the cylinder is completely black inside.
                let flip = if t1 > 0.0 { -1.0 } else { 1.0 };
                let norm = (point - self.start_cap - V * m).norm() * flip;
                return Some(Intersection::new(t2, self.material.clone(), point, norm));
            }
        }

        None // :(
    }

    /// Returns a Vec containing all possible intersections of a Ray and a Shape.
    fn intersect_all(&self, ray: Ray) -> Vec<Intersection> {
        let mut vec = Vec::new();

        let D = ray.direction;
        let V = self.axis;
        let X = ray.origin - self.start_cap;

        // Cylinder intersection points are solutions to at^2+bt+c=0.
        let a = 1.0 - (D * V).powi(2);
        let b = 2.0 * (D * X - (D * V) * (X * V));
        let c = X * X - (X * V).powi(2) - self.radius.powi(2);

        let t1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t1 > 0.0 {
            let m = (D * V) * t1 + X * V;

            if m > 0.0 && m < self.height {
                let point = ray.origin + D * t1;
                let norm_point = point - self.start_cap;
                let norm = (norm_point - norm_point.project(V)).norm();
                vec.push(Intersection::new(t1, self.material.clone(), point, norm));
            }
        }

        let t2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t2 > 0.0 {
            let m = (D * V) * t2 + X * V;

            if m > 0.0 && m < self.height {
                let point = ray.origin + D * t2;
                // we define a factor to flip the normal if we are looking in the cylinder from
                // within the cap hole. Otherwise, the cylinder is completely black inside.
                let flip = if t1 > 0.0 { -1.0 } else { 1.0 };
                let norm = (point - self.start_cap - V * m).norm() * flip;
                vec.push(Intersection::new(t2, self.material.clone(), point, norm));
            }
        }

        vec
    }

    /// Sets the material of the Shape.
    fn set_material(&mut self, material: Arc<Material>) {
        self.material = material;
    }

    /// Gets the material of the Shape.
    fn get_material(&self) -> Arc<Material> {
        self.material.clone()
    }
}
