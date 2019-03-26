mod camera;
mod hitable;
mod material;
mod scene;
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
/// current_depth is the number of times this Ray has bounced off of something
fn color(ray: &Ray, world: &HitableList, current_depth: u32, max_depth: u32) -> Vector3 {
    // 0.001 to correct for rays bouncing off at minimal floats (0.00000000001)
    if let Some(hit_record) = world.hit(&ray, 0.001, std::f32::MAX) {
        if current_depth < max_depth {
            if let Some(scatter_material) = hit_record.material.scatter(ray, &hit_record) {
                return scatter_material.albedo * color(&scatter_material.ray, world, current_depth + 1, max_depth);
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
    let max_depth = 50;
    eprintln!("Generating scene");
    let world = scene::random_scene();
    eprintln!("Generated scene");
    let aspect_ratio = num_rows as f32 / num_cols as f32;
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let v_up = Vector3::new(0.0, 1.0, 0.0);
    let focus_distance = 10f32;
    let camera = Camera::new(look_from, look_at, v_up, 20.0, aspect_ratio, 0.1, focus_distance);

    println!("P3\n{} {}\n255", num_rows, num_cols);
    for y in (0..num_cols).rev() {
        for x in 0..num_rows {
            let mut aa_pixel = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..num_aa_samples {
                let u = (x as f32 + random::<f32>()) / num_rows as f32;
                let v = (y as f32 + random::<f32>()) / num_cols as f32;
                let ray = camera.get_ray(u, v);
                let color = self::color(&ray, &world, 0, max_depth);
                aa_pixel += color;
            }

            aa_pixel /= num_aa_samples as f32;
            let gamma_corrected_pixel = Vector3::new(aa_pixel.r().sqrt(), aa_pixel.g().sqrt(), aa_pixel.b().sqrt());
            let ir = (255.99 * gamma_corrected_pixel.r()) as i32;
            let ig = (255.99 * gamma_corrected_pixel.g()) as i32;
            let ib = (255.99 * gamma_corrected_pixel.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }

        let percent_done = 100.0 * (1.0 - (y as f32 / num_cols as f32));
        eprintln!("{}% done", percent_done);
    }
    eprintln!("Finished generating raytraced image with dimensions X: {}, Y: {}", num_rows, num_cols);
}
