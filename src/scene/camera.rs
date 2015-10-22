use geom::vec3::Vec3;

/*
 * Camera is the central point in the scene from which the rays
 * are emitted.
 *
 * Camera::new(location, direction, up, fov)
 */
pub struct Camera {
    location: Vec3,
    x: Vec3,
    y: Vec3,
    z: Vec3,
    distance: f64
}

impl Camera {
    /*
     * The constructor for camera takes a location, direction,
     * up vector, and field of vision measurement in radians.
     *
     * Distance is calculated by an ugly formula given in Lab 4 Advanced C++.
     */
    fn new(location: Vec3, direction: Vec3, up: Vec3, fov: f64) -> Camera {
        Camera {location: location,
                x: direction.norm(),
                y: (direction % up).norm(),
                z: (y % direction).norm(),
                distance: 0.5 / tan(fov / 2.0)}
    }
}
