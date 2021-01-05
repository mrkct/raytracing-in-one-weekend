pub mod lambertian;
pub mod metal;

use crate::{vec3::Vec3, ray::Ray, hittable::HitRecord};


pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}