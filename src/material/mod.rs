use crate::Vector3;
use crate::HitRecord;
use crate::Ray;

pub use self::dialetric::*;
pub use self::lambertarian::*;
pub use self::metal::*;

mod dialetric;
mod lambertarian;
mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScatter>;
}

pub struct MaterialScatter {
    pub ray: Ray,
    pub albedo: Vector3
}