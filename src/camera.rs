use crate::Vector3;
use crate::Ray;

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
}

impl Camera {
    pub fn new(vfov_deg: f32, aspect_ratio: f32) -> Camera {
        // book prefers passing in degrees and determining radians
        let theta_radians = vfov_deg * std::f32::consts::PI / 180f32;

        // We're trying to find the "Center" of where our camera is pointing at
        // We know the field_of_view so its just finding the "Opposite" line
        // Only the center is half of that line
        let half_height = (theta_radians / 2.0).tan();

        // The aspect ratio will be something like "16:9" -- so its simple to derive width from height
        let half_width = aspect_ratio * half_height;

        Camera {
            lower_left_corner: Vector3::new(-half_width, -half_height, -1.0),
            horizontal: Vector3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0 * half_height, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    /// Given row u and column v, returns a ray broadcasting "Into" the negative Z axis (away from Camera)
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = &self.lower_left_corner + &(&(&self.horizontal * u) + &(&self.vertical * v));
        Ray::new(self.origin.clone(), direction)
    }
}