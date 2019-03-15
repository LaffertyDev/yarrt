mod vector;
mod ray;

use vector::Vec3;
use ray::Ray;

fn is_in_sphere(ray: &Ray, sphere_center: &Vec3, radius: f32) -> bool {
    // The book was really hard to unpack for this part. I struggled unpacking the algebra
    // Basically, we're determining if the "Ray" along ALL values of T will hit a sphere at point "sphere_center"

    // We have a formula for "is this point (x,y,z) on a sphere surface" which is given as X*X + Y*Y + Z*Z = R * R
    // Then we determine the formula for "is this point P (x,y,z) on a sphere surface centered at sphere_center", given as (point - sphere_center) = R*R
        // note, book rewrites this into dot format
        // this is handy, given a "Dot Product" is "The relevant part of the vector applied to another vector"
        // so Dot(point - sphere_center, point - sphere_center) = R*R
    // Where "R" is the radius of the sphere

    // so, a ray is ray_point(t) = ray_origin_vec + ray_direction_vec * t
    // we now want to plug that into the formula, essentially getting us "Given time T, does this ray hit this sphere"
    // substituting the ray formula for our "point", we get a new formula
    // dot ( ray__origin_vec + ray_direction_vec * t - sphere_center, ray_origin_vec + ray_direction_vec * t - sphere_center)
    // which, expanded, gives us a quadratic formula!
    // t^2 * dot (ray_direction_vec, ray_direction_vec) + 2*t * dot ( ray_direction_vec, sphere_center - ray_origin_vec) + dot ( ray_origin_vec, ray_origin_vec) - R^2 = 0
    //                  THIS IS A                                       THIS IS B                                                   THIS IS C
    // (Note, we moved R to set equation to 0)

    // I forgot about the quadratic behavior, but reading up on wikipedia got me back up to speed: https://en.wikipedia.org/wiki/Quadratic_formula#Geometrical_significance
    // Essentially, the number of solutions to the formula are given by b^2 - 4ac
    let sphere_origin_vector = sphere_center - &ray.origin();
    let a = Vec3::dot(&ray.direction(), &ray.direction());
    let b = 2f32 * Vec3::dot(ray.direction(), &sphere_origin_vector);
    let c = Vec3::dot(&sphere_origin_vector, &sphere_origin_vector) - radius * radius;

    let determinant = b*b - 4f32 * a * c;
    // negative means no real solution
    // 0 means the ray does not hit Sphere at center vector with radius
    // more than 0 means that many hits (frankly, one or two)
    determinant > 0f32
}

fn color(ray: &Ray) -> Vec3 {
    if self::is_in_sphere(ray, &Vec3::new(0f32, 0f32, -1f32), 0.5) {
        return Vec3::new(1f32, 0f32, 0f32);
    }

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
