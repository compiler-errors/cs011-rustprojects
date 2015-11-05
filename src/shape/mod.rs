pub mod shape;
pub mod intersection;
pub mod light;
pub mod point_light;
pub mod direction_light;
pub mod material;
pub mod sphere;
pub mod plane;

pub use shape::shape::Shape;
pub use shape::intersection::Intersection;
pub use shape::light::Light;
pub use shape::point_light::PointLight;
pub use shape::direction_light::DirectionLight;
pub use shape::material::Material;
pub use shape::sphere::Sphere;
pub use shape::plane::Plane;
