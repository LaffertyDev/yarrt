use crate::HitRecord;
use crate::Material;
use crate::MaterialScatter;
use crate::Ray;
use crate::Vector3;


pub struct LambertarianMaterial {
    /// Albedo is how much energy is "absorbed" by the material
    /// Higher albedo means more absorbtion
    albedo: Vector3,
}

impl LambertarianMaterial {
    pub fn new(albedo: Vector3) -> Self {
        LambertarianMaterial {
            albedo: albedo
        }
    }
}

impl Material for LambertarianMaterial {
    fn scatter<'a>(&self, ray: &'a Ray, hit_record: &HitRecord) -> Option<MaterialScatter> {
        let target: Vector3 = &hit_record.point + &hit_record.normal + Vector3::random_in_unit_sphere();
        let target_direction = &target - &hit_record.point;

        return Some(
            MaterialScatter {
                ray: Ray::new(hit_record.point.clone(), target_direction),
                albedo: &self.albedo
            }
        )
    }
}

