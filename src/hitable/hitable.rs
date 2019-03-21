use crate::ray::Ray;
use crate::HitRecord;

/// Base trait for defining "Hittable" items
pub trait Hitable {
    /// Determines if the ray will hit this "Hittable" given a range of T
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}