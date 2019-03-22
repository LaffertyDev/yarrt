use crate::Vector3;
use crate::HitRecord;
use crate::Ray;


pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScatter>;
}

pub struct MaterialScatter<'a> {
    pub ray: Ray,
    pub albedo: &'a Vector3
}

pub struct LambertarianMaterial {
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

