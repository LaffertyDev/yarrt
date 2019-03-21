use crate::hitable::hitable::{Hitable};
use crate::HitRecord;
use crate::Ray;
use crate::Vector3;

pub struct Sphere {
    center: Vector3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Sphere {
            center: center,
            radius: radius
        }
    }

    pub fn center(&self) -> &Vector3 {
        return &self.center;
    }

    pub fn radius(&self) -> f32 {
        return self.radius;
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        let sphere_origin_vector = ray.origin() - self.center();
        let a = Vector3::dot(&ray.direction(), &ray.direction());
        let b = Vector3::dot(ray.direction(), &sphere_origin_vector);
        let c = Vector3::dot(&sphere_origin_vector, &sphere_origin_vector) - self.radius() * self.radius();

        let discriminant = b*b - a * c;
        // negative means no real solution
        // 0 means the ray does not hit Sphere at center vector with radius
        // more than 0 means that many hits (frankly, one or two)
        if discriminant < 0f32 {
            return None;
        }

        let time_at_hit = (-b - discriminant.sqrt()) / a;
        if t_min < time_at_hit && time_at_hit < t_max {
            let point_at_hit = ray.point_at_time(time_at_hit);
            let normal = &(&point_at_hit - self.center()) / self.radius();
            let hr = HitRecord::new(time_at_hit, point_at_hit, normal);
            return Some(hr);
        }

        let time_at_hit = (-b + discriminant.sqrt()) / a;
        if t_min < time_at_hit && time_at_hit < t_max {
            let point_at_hit = ray.point_at_time(time_at_hit);
            let normal = &(&point_at_hit - self.center()) / self.radius();
            let hr = HitRecord::new(time_at_hit, point_at_hit, normal);
            return Some(hr);
        }

        return None;
    }
}