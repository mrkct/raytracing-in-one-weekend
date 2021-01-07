use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct Camera {
    origin: Vec3, 
    lower_left_corner: Vec3, 
    horizontal: Vec3, 
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vertical_fov: f64, aspect_ratio: f64) -> Camera {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom.clone();
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Camera { origin, horizontal, vertical, lower_left_corner}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin, 
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        )
    }
}