use crate::HitRecord;
use crate::Material;
use crate::MaterialScatter;
use crate::Ray;
use crate::Vector3;

pub struct MetalMaterial {
    albedo: Vector3,
}

impl MetalMaterial {
    pub fn new(albedo: Vector3) -> Self {
        MetalMaterial {
            albedo: albedo
        }
    }
}

impl Material for MetalMaterial {
    fn scatter<'a>(&self, ray: &'a Ray, hit_record: &HitRecord) -> Option<MaterialScatter> {
        let reflected_direction = Vector3::reflect(&Vector3::unit_vector(ray.direction()), &hit_record.normal);
        let scattered_ray = Ray::new(hit_record.point.clone(), reflected_direction);

        if Vector3::dot(scattered_ray.direction(), &hit_record.normal) < 0f32 {
            return None;
        }
        
        return Some(
            MaterialScatter {
                ray: scattered_ray,
                albedo: &self.albedo
            }
        )
    }
}

