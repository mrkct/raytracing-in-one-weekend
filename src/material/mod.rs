pub mod lambertian;
pub mod metal;
pub mod dielectric;

use crate::{vec3::Vec3, ray::Ray, hittable::HitRecord};


pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = Vec3::dot(&-uv, &n).min(1.0);
    let r_out_perpendicular = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = n * -((1.0 - r_out_perpendicular.length_squared()).abs()).sqrt();
    r_out_perpendicular + r_out_parallel
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2. * Vec3::dot(v, n) * n
}