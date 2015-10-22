extern crate lux;
use lux::geom::vec3::Vec3;
use lux::geom::ray::Ray;
use lux::img::img::Image;

fn main() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);

    print!("{} + {} = {}\n", a, b, a + b);
    print!("{} × {} = {}\n", a, b, a % b);
    print!("-{} = {}\n", a, -a);
    print!("magnitude {} = {}\n", a, a.mag());
    print!("norm {} = {}\n", (a+b), (a+b).norm());

    let c = Ray::between(a, b, true);
    print!("{}", c);
}
