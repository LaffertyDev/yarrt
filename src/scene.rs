
use crate::HitableList;
use crate::material::*;
use crate::hitable::*;
use crate::Vector3;

use rand::prelude::*;


pub fn random_scene() -> HitableList{
    let mut list: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vector3::new(0f32, -1000.0, 0f32), 1000f32, Box::new(LambertarianMaterial::new(Vector3::new(0.5, 0.5, 0.5)))))
    ];

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let material = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());

            if (&center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material: Box<Material> = match material {
                    x if x.in_range(0.0, 0.8) => {
                        Box::new(LambertarianMaterial::new(Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>())))
                    },
                    x if x.in_range(0.8, 0.95) => {
                        Box::new(MetalMaterial::new(Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>()))
                    },
                    _ => {
                        Box::new(DialetricMaterial::new(1.5))
                    },
                };

                let sphere = Sphere::new(center, 0.2, material);
                list.push(Box::new(sphere));
            }
        }
    }


    list.push(Box::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Box::new(DialetricMaterial::new(1.5)))));
    list.push(Box::new(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Box::new(LambertarianMaterial::new(Vector3::new(0.4, 0.2, 0.1))))));
    list.push(Box::new(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Box::new(MetalMaterial::new(Vector3::new(0.7, 0.6, 0.5), 0.0)))));
    return HitableList::new(list);
}

// https://stackoverflow.com/questions/49037111/alternatives-to-matching-floating-point-ranges
trait InRange {
    fn in_range(&self, begin: Self, end: Self) -> bool;
}

impl InRange for f32 {
    fn in_range(&self, begin: f32, end: f32) -> bool {
        *self >= begin && *self < end
    }
}