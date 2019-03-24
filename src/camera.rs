use crate::Vector3;
use crate::Ray;

pub struct Camera {
    /// The aperture of the lens for the Camera, used in determining the depth-of-field / defocus distance
    aperture: f32,

    /// The depth the camera will use in determining the blur of something. This is what the camera is "Focused On"
    // focus_dist: f32,

    /// The lower-left-corner of the Camera, used to determine all other points relative to it
    lower_left_corner: Vector3,

    /// How wide the camera is
    horizontal: Vector3,

    /// How tall the camera is
    vertical: Vector3,

    /// Where the camera is located in 3d space
    origin: Vector3,

    // w is the vector from where we are to where we're looking at, so what we're looking at is always "forward" or "into" the camera, or the Z-component
    // w: Vector3,
    // u is the horizontal vector component defining the left & right axis (X-component) of the camera
    u: Vector3,
    // v is the normalized up & down axis (Y-component)
    v: Vector3,
}

impl Camera {
    /// Not sure why we have "V_UP" instead of a "Roll"
    pub fn new(look_from: Vector3, look_at: Vector3, v_up: Vector3, vfov_deg: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera {

        // Get the orthonormal vectors in camera space
        let w = Vector3::unit_vector(&(&look_from - &look_at));
        let u = Vector3::unit_vector(&v_up.cross(&w));
        let v = w.cross(&u);

        // book prefers passing in degrees and determining radians
        let theta_radians = vfov_deg * std::f32::consts::PI / 180f32;

        // We're trying to find the "Center" of where our camera is pointing at
        // We know the field_of_view so its just finding the "Opposite" line
        // Only the center is half of that line
        let half_height = (theta_radians / 2.0).tan();

        // The aspect ratio will be something like "16:9" -- so its simple to derive width from height
        let half_width = aspect_ratio * half_height;

        let horizontal = half_width * focus_dist * &u;
        let vertical = half_height * focus_dist * &v;

        Camera {
            aperture: aperture,
            // focus_dist: focus_dist,
            lower_left_corner: &look_from - &horizontal - &vertical - focus_dist * &w,
            horizontal: 2.0 * horizontal,
            vertical: 2.0 * vertical,
            origin: look_from,
            // w: w,
            u: u,
            v: v,
        }
    }

    /// Given row u and column v, returns a ray broadcasting "Into" the negative Z axis (away from Camera)
    pub fn get_ray(&self, row: f32, column: f32) -> Ray {
        // why is this random?
        let lens_disk = (self.aperture / 2f32) * Vector3::random_in_disk();
        let offset = &self.u * lens_disk.x() + &self.v * lens_disk.y();

        let ray_origin = &self.origin + offset;
        let direction = &self.lower_left_corner + &(&(&self.horizontal * row) + &(&self.vertical * column)) - &ray_origin;
        Ray::new(ray_origin, direction)
    }
}