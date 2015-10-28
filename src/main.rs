extern crate lux;
use lux::geom::Vec3;
use lux::geom::Ray;
use lux::geom::Color;
use lux::img::Image;
use lux::shape::Sphere;
use lux::shape::Shape;

fn main() {
    test_rays();
}

fn test_rays() {
    let sphere = Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0, Color::black());
    let ray = Ray::between(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0), true);

    if let Some(intersection) = sphere.intersect_first(ray) {
        print!("The ray intersects at {}, which is {} far away\n", intersection.point,
                intersection.distance);
        print!("The normal is pointing {}\n", intersection.norm);
    }
}
