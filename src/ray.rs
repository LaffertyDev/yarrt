use crate::vector::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn point_at_time(&self, t: f32) -> Vec3 {
        (&self.origin + &(&self.direction * t))
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
