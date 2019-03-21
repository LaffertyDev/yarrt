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
