use crate::ray::Ray;
use crate::Vec3;

pub struct HitRecord {
    pub hit_at: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

/// Represents a Ray Hit Record
impl HitRecord {
    // Creates a new Ray Hit Record
    pub fn new(hit_at: f32, point: Vec3, normal: Vec3) -> Self {
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