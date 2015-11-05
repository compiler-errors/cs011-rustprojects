use ::Rc;
use std::f64::INFINITY;
use geom::Vec3;
use geom::Color;
use geom::Ray;
use shape::Shape;
use shape::Intersection;
use shape::Material;

pub struct Cylinder {
    start_cap: Vec3,
    axis: Vec3,
    radius: f64,
    height: f64,
    material: Rc<Material>
}

impl Cylinder {
    pub fn new(start_cap: Vec3, axis: Vec3, radius: f64, height: f64, material: Rc<Material>) ->
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

        let a = 1.0 - (D * V).powi(2);
        let b = 2.0 * (D * X - (D * V) * (X * V));
        let c = X * X - (X * V).powi(2) - self.radius.powi(2);

        let t1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t1 > 0.0 {
            let m = (D * V) * t1 + X * V;

            if m >= 0.0 && m <= self.height {
                let point = ray.origin + D * t1;
                let norm = (point - self.start_cap - V * m).norm();
                return Some(Intersection::new(t1, self.material.clone(), point, norm));
            }
        }

        let t2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t2 > 0.0 {
            let m = (D * V) * t2 + X * V;

            if m >= 0.0 && m <= self.height {
                let point = ray.origin + D * t2;
                // we define a factor to flip the normal if we are looking in the cylinder from
                // within the cap hole. Otherwise, the cylinder is completely black inside.
                let flip = if t1 > 0.0 { -1.0 } else { 1.0 };
                let norm = (point - self.start_cap - V * m).norm() * flip;
                return Some(Intersection::new(t2, self.material.clone(), point, norm));
            }
        }

        //check if the point intersects with any of the end-caps.
        /*TODO: Fix, maybe replace with disk intersection code.
        if t1 > 0.0 || t2 > 0.0 {
            let mut tbest = INFINITY;
            let mut ibest: Option<Intersection> = None;

            let Vp = -V;
            let det1 = D * Vp;

            if (det1 != 0.0) {
                let t = -X * Vp / det1;

                if t >= 0.0 {
                    tbest = t;
                    ibest = Some(Intersection::new(t, self.material.clone(), ray.origin + D * t,
                                Vp));
                }
            }

            //let X2 = ray.origin - (self.start_cap + V * self.height);
            //let det2 = D * V;

            //if (det2 != 0.0) {
            //    let t = -X2 * V / det2;

            //    if t >= 0.0 && t < tbest {
            //        return Some(Intersection::new(t, self.material.clone(), ray.origin + D * t,
            //                    V));
            //    }
            //}

            return ibest;
        }*/

        None // :(
    }

    /// Returns a Vec containing all possible intersections of a Ray and a Shape.
    fn intersect_all(&self, ray: Ray) -> Vec<Intersection> {
        let mut vec = Vec::new();

        let D = ray.direction;
        let V = self.axis;
        let X = ray.origin - self.start_cap;

        let a = 1.0 - (D * V).powi(2);
        let b = 2.0 * (D * X - (D * V) * (X * V));
        let c = X * X - (X * V).powi(2) - self.radius.powi(2);

        let t1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t1 > 0.0 {
            let m = (D * V) * t1 + X * V;

            if m >= 0.0 && m <= self.height {
                let point = ray.origin + D * t1;
                let norm = (point - self.start_cap - V * m).norm();
                vec.push(Intersection::new(t1, self.material.clone(), point, norm));
            } else { //TODO: "else if capped"
                let Vp = -V;
                let det = D * Vp;

                if (det != 0.0) {
                    let t = -X * Vp / det;

                    if t >= 0.0 {
                        vec.push(Intersection::new(t, self.material.clone(), ray.origin + D * t,
                                 Vp));
                    }
                }
            }
        }

        let t2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t2 > 0.0 {
            let m = (D * V) * t2 + X * V;

            if m >= 0.0 && m <= self.height {
                let point = ray.origin + D * t2;
                let norm = (point - self.start_cap - V * m).norm();
                vec.push(Intersection::new(t2, self.material.clone(), point, norm));
            } else { //TODO: "else if capped"
                let X2 = ray.origin - (self.start_cap + V * self.height);
                let det = D * V;

                if (det != 0.0) {
                    let t = -X2 * V / det;

                    if t >= 0.0 {
                        vec.push(Intersection::new(t, self.material.clone(), ray.origin + D * t,
                                 V));
                    }
                }
            }
        }

        vec
    }

    /// Sets the material of the Shape.
    fn set_material(&mut self, material: Rc<Material>) {
        self.material = material;
    }

    /// Gets the material of the Shape.
    fn get_material(&self) -> Rc<Material> {
        self.material.clone()
    }
}
