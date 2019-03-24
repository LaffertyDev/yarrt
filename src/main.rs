mod camera;
mod hitable;
mod material;
mod ray;
mod vector;

extern crate rand;

use camera::Camera;
use hitable::*;
use material::*;
use ray::Ray;
use vector::Vector3;

use rand::prelude::*;

/// Determines the Color that this Ray should have in the world
/// First we determine the nearest object the ray is going to hit in the world
/// If it hit, and we haven't reached the max hit-depth, re-cast the ray from the hitable's geometric material
/// If it hit, and we have reached the max hit-depth, the "base-color" is solid black (0,0,0)
/// If it did not hit, then the Ray has reflected into the background
/// 
/// Depth is the "Current Depth" of the recursive color
fn color(ray: &Ray, world: &HitableList, depth: i32) -> Vector3 {
    // 0.001 to correct for rays bouncing off at minimal floats (0.00000000001)
    if let Some(hit_record) = world.hit(&ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(scatter_material) = hit_record.material.scatter(ray, &hit_record) {
                return scatter_material.albedo * color(&scatter_material.ray, world, depth + 1);
            }
        }

        // passed the depth or we've attenuated the ray
        return Vector3::new(0.0, 0.0, 0.0);
    } else {
        // did not hit an object, return the background
        let ray_direction_unit = Vector3::unit_vector(&ray.direction());
        let t = 0.5f32 * ray_direction_unit.y() + 1f32;
        return &(&Vector3::new(1.0, 1.0, 1.0) * (1f32 - t)) + &(&Vector3::new(0.5, 0.7, 1.0) * t);
    }
}

fn main() {
    let num_rows = 200;
    let num_cols = 100;
    let num_aa_samples = 100;

    // let list:Vec<Box<Hitable>> = vec![
    //      Box::new(Sphere::new(Vector3::new(-sphere_radius, 0f32, -1f32), sphere_radius, Box::new(LambertarianMaterial::new(Vector3::new(0.0, 0.0, 1.0))))),
    //      Box::new(Sphere::new(Vector3::new(sphere_radius, 0f32, -1f32), sphere_radius, Box::new(LambertarianMaterial::new(Vector3::new(1.0, 0.0, 0.0))))),
    // ];

    let list: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vector3::new(0f32, 0f32, -1f32), 0.5, Box::new(LambertarianMaterial::new(Vector3::new(0.0, 0.0, 1.0))))),
        Box::new(Sphere::new(Vector3::new(0f32, -100.5, -1f32), 100f32, Box::new(LambertarianMaterial::new(Vector3::new(0.8, 0.8, 0.0))))),
        Box::new(Sphere::new(Vector3::new(1f32, 0f32, -1f32), 0.5, Box::new(MetalMaterial::new(Vector3::new(0.8, 0.6, 0.2), 0.5)))),
        Box::new(Sphere::new(Vector3::new(-1f32, 0f32, -1f32), 0.5, Box::new(DialetricMaterial::new(1.5)))),
        Box::new(Sphere::new(Vector3::new(-1f32, 0f32, -1f32), -0.45, Box::new(DialetricMaterial::new(1.5)))),
    ];

    let world = HitableList::new(list);
    let aspect_ratio = num_rows as f32 / num_cols as f32;
    let look_from = Vector3::new(3.0, 3.0, 2.0);
    let look_at = Vector3::new(0.0, 0.0, -1.0);
    let v_up = Vector3::new(0.0, 1.0, 0.0);
    let focus_distance = (&look_from - &look_at).magnitude();
    let camera = Camera::new(look_from, look_at, v_up, 20.0, aspect_ratio, 2.0, focus_distance);

    println!("P3\n{} {}\n255", num_rows, num_cols);
    for y in (0..num_cols).rev() {
        for x in 0..num_rows {
            let mut aa_pixel = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..num_aa_samples {
                let u = (x as f32 + random::<f32>()) / num_rows as f32;
                let v = (y as f32 + random::<f32>()) / num_cols as f32;
                let ray = camera.get_ray(u, v);
                let color = self::color(&ray, &world, 0i32);
                aa_pixel += color;
            }

            aa_pixel /= num_aa_samples as f32;
            let gamma_corrected_pixel = Vector3::new(aa_pixel.r().sqrt(), aa_pixel.g().sqrt(), aa_pixel.b().sqrt());
            let ir = (255.99 * gamma_corrected_pixel.r()) as i32;
            let ig = (255.99 * gamma_corrected_pixel.g()) as i32;
            let ib = (255.99 * gamma_corrected_pixel.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
