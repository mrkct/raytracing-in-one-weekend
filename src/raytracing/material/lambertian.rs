use crate::vec3::Vec3;
use crate::raytracing::{
    material::Material, 
    ray::Ray,  
    hittable::HitRecord
};


pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        
        let attenuation = self.albedo;

        let scatter_direction = {
            let dir = hit_record.normal + Vec3::random_unit_vector();
            if !dir.near_zero() { 
                dir
            } else {
                hit_record.normal
            }
        };
        let scattered_ray = Ray::new(hit_record.p, scatter_direction);

        Some((attenuation, scattered_ray))
    }
}