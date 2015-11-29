use std::f64::consts::FRAC_PI_3;
use std::sync::Arc;

extern crate lux;
use lux::geom::Vec3;
use lux::geom::Color;
use lux::shape::*;
use lux::scene::Camera;
use lux::scene::World;

fn main() {
    test_world();
}

fn test_world() {
    let mut world = World::new(Color::new(0.9, 0.9, 0.9));

    let plane_material = Arc::new(Material::lambertian(Color::new(1.0, 1.0, 1.0)));
    let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0, plane_material);
    let sphere1_material = Arc::new(Material::lambertian(Color::new(1.0, 0.0, 0.0)));
    let sphere2_material = Arc::new(Material::glossy(Color::new(0.7, 1.0, 0.7), 1000.0));
    let sphere3_material = Arc::new(Material::glossy(Color::new(0.3, 0.3, 1.0), 10.0));
    let sphere1 = Sphere::new(Vec3::new(-1.2, 0.5, 0.0), 0.5, sphere1_material);
    let sphere2 = Sphere::new(Vec3::new(0.0, 0.5, 0.0), 0.5, sphere2_material);
    let sphere3 = Sphere::new(Vec3::new(1.2, 0.5, 0.0), 0.5, sphere3_material);

    let light1 = DirectionLight::new(Vec3::new(1.0, -1.0, 0.0), Color::new(0.8, 0.8, 0.8));
    let light2 = PointLight::new(Vec3::new(10.0, 10.0, 3.0), Color::new(1.0, 1.0, 1.0));

    let camera = Camera::new(Vec3::new(-1.5, 1.0, 3.0), Vec3::new(-0.3, 0.5, 0.0),
                             Vec3::up(), FRAC_PI_3*0.9, 400, 400);

    world.add_shape(Arc::new(plane));
    world.add_shape(Arc::new(sphere1));
    world.add_shape(Arc::new(sphere2));
    world.add_shape(Arc::new(sphere3));

    //world.add_light(Arc::new(light1));
    //world.add_light(Arc::new(light2));

    let image = camera.trace_image(&world);
    image.save("image.ppm");
}
