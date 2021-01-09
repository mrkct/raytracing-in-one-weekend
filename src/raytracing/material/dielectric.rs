use crate::vec3::Vec3;
use crate::raytracing::{
    material::Material,
    material, 
    ray::Ray,  
    hittable::HitRecord
};

pub struct Dielectric {
    refractive_index: f64
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric {refractive_index}
    }
}

fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::ONE;
        let refraction_ratio = {
            if hit_record.front_face { 1.0 / self.refractive_index } else { self.refractive_index }
        };

        let unit_direction = ray_in.direction().unit_vector();
        
        let cos_theta = Vec3::dot(&-unit_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = {
            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random() {
                material::reflect(&unit_direction, &hit_record.normal)
            } else {
                material::refract(&unit_direction, &hit_record.normal, refraction_ratio)
            }
        };
        
        let scattered = Ray::new(hit_record.p, direction);

        Some((attenuation, scattered))
    }
}