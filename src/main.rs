use std::f64::consts::FRAC_PI_3;

extern crate lux;
use lux::geom::Vec3;
use lux::geom::Ray;
use lux::geom::Color;
use lux::img::Image;
use lux::shape::Shape;
use lux::shape::Sphere;
use lux::shape::Plane;
use lux::shape::PointLight;
use lux::scene::Camera;
use lux::scene::World;

fn main() {
    test_world();
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

fn test_world_simple() {
    let mut world = World::new();

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Color::new(1.0, 0.0, 0.0));
    let light = PointLight::new(Vec3::new(-5.0, 10.0, -5.0), Color::new(0.8, 0.8, 0.8));

    world.add_shape(Box::new(sphere));
    world.add_light(Box::new(light));

    let camera = Camera::new(Vec3::new(-5.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0),
                             Vec3::new(0.0, 1.0, 0.0), FRAC_PI_3, 400, 400);

    let image = camera.trace_image(&world);
    image.save("image.ppm");
}

fn test_world() {
    let mut world = World::new();

    let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0, Color::new(0.5, 0.0, 0.5));
    let sphere1 = Sphere::new(Vec3::new(-1.2, 0.5, 0.0), 0.5, Color::new(1.0, 0.0, 0.0));
    let sphere2 = Sphere::new(Vec3::new(0.0, 0.5, 0.0), 0.5, Color::new(0.0, 1.0, 0.0));
    let sphere3 = Sphere::new(Vec3::new(1.2, 0.5, 0.0), 0.5, Color::new(0.0, 0.0, 1.0));

    let light1 = PointLight::new(Vec3::new(-10.0, 10.0, 5.0), Color::new(0.8, 0.8, 0.8));
    let light2 = PointLight::new(Vec3::new(5.0, 3.0, 5.0), Color::new(0.3, 0.3, 0.3));

    let camera = Camera::new(Vec3::new(-1.5, 1.0, 3.0), Vec3::new(-0.3, 0.5, 0.0),
                             Vec3::new(0.0, 1.0, 0.0), FRAC_PI_3, 400, 400);

    world.add_shape(Box::new(plane));
    world.add_shape(Box::new(sphere1));
    world.add_shape(Box::new(sphere2));
    world.add_shape(Box::new(sphere3));

    world.add_light(Box::new(light1));
    world.add_light(Box::new(light2));

    let image = camera.trace_image(&world);
    image.save("image.ppm");
}
