use crate::vector::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    // Rust is forcing me to take ownership of the "origin" and "Direction" vectors
    // Meaning once this method is called, nothing can reuse them
    // so if I tried, I'd get a compilation error
    /*

    let origin = Vec3 {...}
    let dest = Vec3 {...}
    let ray = Ray::new(origin, dest);
    if origin // ERROR -- ray owns origin now
    */
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    // creates a NEW vec3 so whoever calls this method has ownership of it, without changing my origin vector
    pub fn point_at_time(&self, t: f32) -> Vec3 {
        (&self.origin * t)
    }

    // returns a READONLY reference to the origin vector
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
