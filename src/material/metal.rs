use crate::HitRecord;
use crate::Material;
use crate::MaterialScatter;
use crate::Ray;
use crate::Vector3;

/// Metalic materials are very glossy, meaning extreme reflectivity
pub struct MetalMaterial {
    /// Albedo is how much energy is "absorbed" by the material
    /// Higher albedo means less absorbtion
    albedo: Vector3,
    /// Reflective materials are naturally "Fuzzy"
    /// At first I didn't believe it, but its real. See this pdf
    /// https://pdfs.semanticscholar.org/7233/759231154dd9c8e8c0a6fdaae64f103ab58f.pdf
    /// 
    /// This effectively causes rays to not reflect "perfectly" which causes the distortion we're looking for
    fuzziness: f32,
}

impl MetalMaterial {
    pub fn new(albedo: Vector3, fuzziness: f32) -> Self {
        MetalMaterial {
            albedo: albedo,
            fuzziness: fuzziness
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScatter> {
        let reflected_direction = Vector3::reflect(&Vector3::unit_vector(ray.direction()), &hit_record.normal);
        let scattered_ray = Ray::new(hit_record.point.clone(), reflected_direction + self.fuzziness * Vector3::random_in_unit_sphere());

        if Vector3::dot(scattered_ray.direction(), &hit_record.normal) < 0f32 {
            return None;
        }
        
        return Some(
            MaterialScatter {
                ray: scattered_ray,
                albedo: self.albedo.clone()
            }
        )
    }
}

