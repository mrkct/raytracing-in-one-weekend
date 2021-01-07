use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct Camera {
    origin: Vec3, 
    lower_left_corner: Vec3, 
    horizontal: Vec3, 
    vertical: Vec3, 

    u: Vec3, 
    v: Vec3, 
    w: Vec3, 
    lens_radius: f64
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3, 
        lookat: &Vec3, 
        vup: &Vec3, 
        vertical_fov: f64, 
        aspect_ratio: f64, 
        aperture: f64, 
        focus_dist: f64
    ) -> Camera {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom.clone();
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        Camera {
            origin, 
            horizontal, 
            vertical, 
            lower_left_corner, 
            w, u, v, lens_radius
        }
    }

    fn random_in_unit_disk() -> (f64, f64) {
        let rand_double = || -1.0 + 2.0 * rand::random::<f64>();

        (rand_double(), rand_double())
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let (rd_x, rd_y) = Camera::random_in_unit_disk();
        let (rd_x, rd_y) = (self.lens_radius * rd_x, self.lens_radius * rd_y);
        let offset = self.u * rd_x + self.v * rd_y;

        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        )
    }
}