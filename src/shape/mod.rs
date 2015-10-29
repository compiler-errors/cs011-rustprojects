pub mod shape;
pub mod intersection;
pub mod point_light;
pub mod sphere;
pub mod plane;

pub use shape::shape::Shape;
pub use shape::intersection::Intersection;
pub use shape::point_light::PointLight;
pub use shape::sphere::Sphere;
pub use shape::plane::Plane;
