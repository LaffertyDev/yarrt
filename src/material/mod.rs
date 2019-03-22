use crate::Vector3;
use crate::HitRecord;
use crate::Ray;

pub use self::lambertarian::*;
pub use self::metal::*;

mod lambertarian;
mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScatter>;
}

pub struct MaterialScatter<'a> {
    pub ray: Ray,
    pub albedo: &'a Vector3
}