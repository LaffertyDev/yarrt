mod camera;
mod hitable;
mod ray;
mod vector;

extern crate rand;

use ray::Ray;
use vector::Vector3;
use hitable::*;
use camera::Camera;

use rand::prelude::*;

fn color(ray: &Ray, world: &HitableList) -> Vector3 {
    // 0.001 to correct for rays bouncing off at minimal floats (0.00000000001)
    if let Some(hit_record) = world.hit(&ray, 0.001, std::f32::MAX) {
        let target: Vector3 = &hit_record.point + &hit_record.normal + Vector3::random_in_unit_sphere();
        let target_direction = &target - &hit_record.point;
        return 0.5 * self::color(&Ray::new(hit_record.point, target_direction), world);
    } else {
        let ray_direction_unit = Vector3::unit_vector(&ray.direction());
        let t = 0.5f32 * ray_direction_unit.y() + 1f32;
        return &(&Vector3::new(1.0, 1.0, 1.0) * (1f32 - t)) + &(&Vector3::new(0.5, 0.7, 1.0) * t);
    }
}

fn main() {
    let num_rows = 200;
    let num_cols = 100;
    let num_aa_samples = 100;

    let list: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vector3::new(0f32, 0f32, -1f32), 0.5)),
        Box::new(Sphere::new(Vector3::new(0f32, -100.5, -1f32), 100f32)),
    ];

    let world = HitableList::new(list);
    let camera = Camera::new();

    println!("P3\n{} {}\n255", num_rows, num_cols);
    for y in (0..num_cols).rev() {
        for x in 0..num_rows {
            let mut aa_pixel = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..num_aa_samples {
                let u = (x as f32 + random::<f32>()) / num_rows as f32;
                let v = (y as f32 + random::<f32>()) / num_cols as f32;
                let ray = camera.get_ray(u, v);
                let color = self::color(&ray, &world);
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
