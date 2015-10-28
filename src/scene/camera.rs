use geom::Vec3;
use scene::World;

/// Camera is the central point in the scene from which the rays are emitted.
///
/// It defines the "film" where the rays are emitted by using a field of vision measurement which
///  calculates an effective focal distance.
pub struct Camera {
    location: Vec3,
    x: Vec3,
    y: Vec3,
    z: Vec3,
    distance: f64
}

impl Camera {
    /// Constructs a Camera struct using a location, direction, up vector, and field of vision
    /// measurement in radians.
    pub fn new(location: Vec3, direction: Vec3, up: Vec3, fov: f64) -> Camera {
        Camera {location: location,
                x: direction.norm(),
                y: (direction % up).norm(),
                z: ((direction % up) % direction).norm(),
                distance: 0.5 / (fov / 2.0).tan()}
    }

    pub fn trace(&self, world: &World) {

    }
}
