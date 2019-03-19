use crate::ray::Ray;
use crate::hitable::hitable::{HitRecord, Hitable};

pub struct HitableList {
    hitables: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new(hitables: Vec<Box<Hitable>>) -> HitableList {
        HitableList {
            hitables: hitables
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_max = t_max;
        let mut closest_hit: Option<HitRecord> = None;
        for hitable in self.hitables.iter() {
            if let Some(hit_record) = hitable.hit(ray, t_min, t_max) {
                t_max = hit_record.hit_at;
                closest_hit = Some(hit_record);
            }
        }
        return closest_hit;
    }
}