mod vector;
mod ray;

use vector::Vec3;
use ray::Ray;

fn color(ray: &Ray) -> Vec3 {
    let ray_direction_unit = Vec3::unit_vector(&ray.direction());
    let t = 0.5f32 * ray_direction_unit.y() + 1f32;

    return &(&Vec3::new(1.0, 1.0, 1.0) * (1f32 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t);
}

fn main() {
    let num_rows = 200;
    let num_cols = 100;

    let lower_left_corner = Vec3::new(-2f32, -1f32, -1f32);
    let horizontal = Vec3::new(4f32, 0f32, 0f32);
    let vertical = Vec3::new(0f32, 2f32, 0f32);

    println!("P3\n{} {}\n255", num_rows, num_cols);
    for y in (0..num_cols).rev() {
        for x in 0..num_rows {
            let u = x as f32 / num_rows as f32;
            let v = y as f32 / num_cols as f32;
            // clone
            // pass read-only reference
            let origin = Vec3::new(0f32, 0f32, 0f32);
            let ray = Ray::new(origin, &lower_left_corner + &(&(&horizontal * u) + &(&vertical * v)));
            let color = self::color(&ray);
            
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
