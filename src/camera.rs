use crate::vec3::{self, Vec3};
use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    time_0: f32,
    time_1: f32
}

impl Camera {
    pub fn new(lookfrom: Vec3, 
        lookat: Vec3, 
        vup: Vec3, 
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32, 
        focus_dist: f32,
        time_0: f32,
        time_1: f32
        ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0; // convert degrees to radians
        let h = (theta/2.0).tan();
        

        // Camera
    	// We setup a virtual viewport through which pass the scene rays.
    	let viewport_height = 2.0 * h;
    	let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
 
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            time_0,
            time_1
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray(self.origin + offset,
            self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset, 
            vec3::random_double(Some(self.time_0), Some(self.time_1)))
    }
}
