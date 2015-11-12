use std::f64::consts::FRAC_PI_3;
use std::rc::Rc;

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
    let mut world = World::new(Color::new(1.0, 1.0, 1.0));

    let plane_material = Rc::new(Material::lambertian(Color::new(0.3, 0.3, 0.3)));
    let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0, plane_material);
    let sphere1_material = Rc::new(Material::lambertian(Color::new(1.0, 0.0, 0.0)));
    let sphere2_material = Rc::new(Material::glossy(Color::new(1.0, 1.0, 1.0), 50.0));
    let cylinder1_material = Rc::new(Material::reflective(Color::new(1.0, 1.0, 0.5)));
    let sphere1 = Sphere::new(Vec3::new(-1.2, 0.5, 0.0), 0.5, sphere1_material);
    let sphere2 = Sphere::new(Vec3::new(0.0, 0.5, 0.0), 0.5, sphere2_material);
    let cylinder1 = Cylinder::new(Vec3::new(1.2, -0.01, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.55, 0.75, cylinder1_material);

    let light1 = DirectionLight::new(Vec3::new(1.0, -1.0, 0.0), Color::new(0.2, 0.2, 0.2));
    //let light2 = PointLight::new(Vec3::new(10.0, 10.0, 3.0), Color::new(0.3, 0.3, 0.3));

    let camera = Camera::new(Vec3::new(-1.5, 1.0, 3.0), Vec3::new(-0.3, 0.5, 0.0),
                             Vec3::up(), FRAC_PI_3, 1000, 1000);

    world.add_shape(Box::new(plane));
    world.add_shape(Box::new(sphere1));
    world.add_shape(Box::new(sphere2));
    world.add_shape(Box::new(cylinder1));

    //world.add_light(Box::new(light1));
    //world.add_light(Box::new(light2));

    let image = camera.trace_image(&world);
    image.save("image.ppm");
}
