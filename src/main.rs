mod hitable;
mod ray;
mod vector;

use ray::Ray;
use vector::Vec3;
use hitable::*;

fn color(ray: &Ray, world: &HitableList) -> Vec3 {
    if let Some(hit_record) = world.hit(&ray, 0f32, std::f32::MAX) {
        let colorized_normal = &(&hit_record.normal + 1f32) / 2f32;
        return colorized_normal;
    } else {
        let ray_direction_unit = Vec3::unit_vector(&ray.direction());
        let t = 0.5f32 * ray_direction_unit.y() + 1f32;
        return &(&Vec3::new(1.0, 1.0, 1.0) * (1f32 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t);
    }
}

fn main() {
    let num_rows = 200;
    let num_cols = 100;

    let lower_left_corner = Vec3::new(-2f32, -1f32, -1f32);
    let horizontal = Vec3::new(4f32, 0f32, 0f32);
    let vertical = Vec3::new(0f32, 2f32, 0f32);

    let list: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0f32, 0f32, -1f32), 0.5)),
        Box::new(Sphere::new(Vec3::new(0f32, -100.5, -1f32), 100f32)),
    ];

    let world = HitableList::new(list);

    println!("P3\n{} {}\n255", num_rows, num_cols);
    for y in (0..num_cols).rev() {
        for x in 0..num_rows {
            let u = x as f32 / num_rows as f32;
            let v = y as f32 / num_cols as f32;
            // clone
            // pass read-only reference
            let origin = Vec3::new(0f32, 0f32, 0f32);
            let ray = Ray::new(origin, &lower_left_corner + &(&(&horizontal * u) + &(&vertical * v)));

            let color = self::color(&ray, &world);
            
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
