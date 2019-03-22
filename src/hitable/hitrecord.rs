use crate::Material;
use crate::Vector3;

pub struct HitRecord<'a> {
    pub hit_at: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Box<&'a dyn Material>,
}

/// Represents a Ray Hit Record
impl<'a> HitRecord<'a> {
    // Creates a new Ray Hit Record
    pub fn new(hit_at: f32, point: Vector3, normal: Vector3, material: Box<&'a dyn Material>) -> Self {
        HitRecord {
            hit_at: hit_at,
            point: point,
            normal: normal,
            material: material,
        }
    }
}
