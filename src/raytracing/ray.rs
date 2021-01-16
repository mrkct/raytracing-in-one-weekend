use crate::vec3::Vec3;
use super::hittable;

pub struct Ray {
    origin: Vec3, 
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    pub fn origin(&self) -> &Vec3 { &self.origin }
    pub fn direction(&self) -> &Vec3 { &self.direction }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin() + t * self.direction()
    }

    pub fn ray_color(&self, world: &Vec<Box<dyn hittable::Hittable + Send + Sync>>, max_depth: i32) -> Vec3 {
        if max_depth <= 0 { return Vec3::ZERO; }
    
        if let Some(hit_record) = hittable::hits(world, self, 0.001, std::f64::INFINITY) {
            
            if let Some((attenuation, scattered)) = hit_record.material.as_ref().scatter(self, &hit_record) {
                return attenuation * scattered.ray_color(world, max_depth - 1);
            }
    
            return Vec3::ZERO;
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (1.0 + unit_direction.y);
        (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
    }
}