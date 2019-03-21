use crate::ray::Ray;
use crate::Vector3;

pub struct HitRecord {
    pub hit_at: f32,
    pub point: Vector3,
    pub normal: Vector3,
}

/// Represents a Ray Hit Record
impl HitRecord {
    // Creates a new Ray Hit Record
    pub fn new(hit_at: f32, point: Vector3, normal: Vector3) -> Self {
        HitRecord {
            hit_at: hit_at,
            point: point,
            normal: normal
        }
    }
}

/// Base trait for defining "Hittable" items
pub trait Hitable {
    /// Determines if the ray will hit this "Hittable" given a range of T
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}