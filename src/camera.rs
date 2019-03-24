use crate::Vector3;
use crate::Ray;

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
}

impl Camera {
    /// Not sure why we have "V_UP" instead of a "Roll"
    pub fn new(look_from: Vector3, look_at: Vector3, v_up: Vector3, vfov_deg: f32, aspect_ratio: f32) -> Camera {

        // Get the orthonormal vectors in camera space
        // w is the vector from where we are to where we're looking at, so what we're looking at is always "forward" or "into" the camera, or the Z-component
        let w = Vector3::unit_vector(&(&look_from - &look_at));
        // u is the horizontal vector component defining the left & right axis (X-component) of the camera
        let u = Vector3::unit_vector(&v_up.cross(&w));
        // v is the normalized up & down axis (Y-component)
        let v = w.cross(&u);

        // book prefers passing in degrees and determining radians
        let theta_radians = vfov_deg * std::f32::consts::PI / 180f32;

        // We're trying to find the "Center" of where our camera is pointing at
        // We know the field_of_view so its just finding the "Opposite" line
        // Only the center is half of that line
        let half_height = (theta_radians / 2.0).tan();

        // The aspect ratio will be something like "16:9" -- so its simple to derive width from height
        let half_width = aspect_ratio * half_height;

        Camera {
            lower_left_corner: &look_from - (half_width * &u) - (half_height * &v) - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from
        }
    }

    /// Given row u and column v, returns a ray broadcasting "Into" the negative Z axis (away from Camera)
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = &self.lower_left_corner + &(&(&self.horizontal * u) + &(&self.vertical * v)) - &self.origin;
        Ray::new(self.origin.clone(), direction)
    }
}