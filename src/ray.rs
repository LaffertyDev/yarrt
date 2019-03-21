use crate::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn point_at_time(&self, t: f32) -> Vector3 {
        (&self.origin + &(&self.direction * t))
    }

    pub fn origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }
}
