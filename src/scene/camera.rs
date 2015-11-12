use ::rand::{random, Closed01};
use geom::Vec3;
use geom::Ray;
use geom::Color;
use scene::World;
use img::Image;

const SAMPLES: i32 = 400;

/// Camera is the central point in the scene from which the rays are emitted.
///
/// It defines the "film" where the rays are emitted by using a field of vision measurement which
///  calculates an effective focal distance.
pub struct Camera {
    location: Vec3,
    x: Vec3, //forward
    y: Vec3, //up
    z: Vec3, //right
    distance: f64,
    width: i32,
    height: i32
}

impl Camera {
    /// Constructs a Camera struct using a location, direction, up vector, and field of vision
    /// measurement in radians.
    pub fn new(location: Vec3, lookat: Vec3, up: Vec3, fov: f64, width: i32, height: i32)
               -> Camera {
        let direction = (lookat - location).norm();
        Camera {location: location,
                x: direction,
                z: (direction % up).norm(),
                y: ((direction % up).norm() % direction).norm(),
                distance: 0.5 / (fov / 2.0).tan(),
                width: width,
                height: height}
    }

    /// Traces an image and saves each pixel onto an image which is returned to the caller.
    pub fn trace_image(&self, world: &World) -> Image {
        let mut image = Image::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let mut color = Color::black();

                for _ in 0..SAMPLES {
                    // get the ray that intersects a specific pixel
                    let ray = self.get_ray_for_pixel((x as f64) + jitter(), (y as f64) + jitter());
                    // trace the ray to get the color visible through the pixel
                    color = color + world.trace_ray(&ray, 0);
                    // save the pixel's color on "film"
                }

                image.set_color(x, y, color * (1.0 / (SAMPLES as f64)));
            }
            print!("{}% done!\n", (x as f64) /
                    (self.width as f64) * 100.0);
        }

        image
    }

    /// Returns a ray which intersects the 2-dimensional pixel (x, y) on the view plane constructed
    /// from our field of vision.
    fn get_ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        assert!(x >= -1.0 && x < (self.width as f64) + 1.0, "Pixel x-coordinate out of bounds!");
        assert!(y >= -1.0 && y < (self.height as f64) + 1.0, "Pixel y-coordinate out of bounds!");

        let pixelDir = self.x * self.distance
                     + self.z * (x / (self.width as f64) - 0.5)
                     + self.y * (0.5 - y / (self.height as f64));

        Ray::new(self.location, pixelDir)
    }
}

/// A helper method which returns a random real on [0, 1].
fn jitter() -> f64 {
    let Closed01(val) = random::<Closed01<f64>>();
    val
}
