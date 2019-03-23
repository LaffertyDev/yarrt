use crate::HitRecord;
use crate::Material;
use crate::MaterialScatter;
use crate::Ray;
use crate::Vector3;

use rand::prelude::*;

pub struct DialetricMaterial {
    refractive_index: f32,
}



impl DialetricMaterial {
    pub fn new(refractive_index: f32) -> Self {
        DialetricMaterial {
            refractive_index: refractive_index
        }
    }

    /// Determines the reflectivity of the material given the angle of approach
    /// 
    /// This is an approximation algorithm
    fn shlick(cosine: f32, refractive_index: f32) -> f32 {
        let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

/// Dialetric materials are like Water -- they both reflect and refract
/// 
/// First priority is finding the "Normal that points outward" from the source of light
/// Then we compute the refraction using Schnell's law
/// Then we determine if we want to return the reflection or the refraction
impl Material for DialetricMaterial {
    fn scatter<'a>(&self, ray: &'a Ray, hit_record: &HitRecord) -> Option<MaterialScatter> {
        let reflected = Vector3::reflect(ray.direction(), &hit_record.normal);
        let refraction_differential: f32;
        let outward_normal: &Vector3;
        let inverted_normal = -&hit_record.normal;
        let cosine: f32;
        if Vector3::dot(ray.direction(), &hit_record.normal) > 0.0 {
            outward_normal = &inverted_normal;
            refraction_differential = self.refractive_index;
            cosine = self.refractive_index * Vector3::dot(ray.direction(), &hit_record.normal) / ray.direction().magnitude();
        } else {
            outward_normal = &hit_record.normal;
            refraction_differential = 1.0 / self.refractive_index;
            cosine = - Vector3::dot(ray.direction(), &hit_record.normal) / ray.direction().magnitude();
        }

        let reflect_probability = DialetricMaterial::shlick(cosine, self.refractive_index);
        let scattered_ray: Ray;
        if random::<f32>() < reflect_probability {
            // reflected
            scattered_ray = Ray::new(hit_record.point.clone(), reflected);
        } else if let Some(refracted) = Vector3::refract(ray.direction(), outward_normal, refraction_differential) {
            scattered_ray = Ray::new(hit_record.point.clone(), refracted);
        } else {
            // rust doesn't yet let you join if lets
            // if we didn't refract we 100% reflected
            scattered_ray = Ray::new(hit_record.point.clone(), reflected);
        }

        return Some(
            MaterialScatter {
                ray: scattered_ray,
                albedo: Vector3::new(1.0, 1.0, 1.0)
            }
        )
    }
}

