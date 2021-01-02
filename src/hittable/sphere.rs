use crate::hittable::{Hittable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct Sphere {
    center: Vec3, 
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        /*
            We know a point is on a sphere at (0, 0, 0) if x^2+y^2+z^2 = r^2
            In this case our ray indicates (x,y,z), do some math and you end up 
            with a 2nd grade equation. If you solve that you get the points that 
            intercepts from the ray origin onto the sphere. 
        
            Consider that in vectors length_squared is equivalent to dot 
            product of self

            Consider that we simplified the expression here, taking the 2s out 
            of the square root and dividing by 2 everything, which is why this 
            is not exactly the same as the quadratic formula
        */ 
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. { return None; }
        let sqrd = discriminant.sqrt();

        let root = {
            let is_valid_root = |x| t_min <= x && x <= t_max;

            let neg_root = (-half_b - sqrd) / a;
            if is_valid_root(neg_root) {
                Some(neg_root)
            } else {
                let pos_root = (-half_b + sqrd) / a;
                if is_valid_root(pos_root) {
                    Some(pos_root)
                } else {
                    None
                }
            }
        };
        if root.is_none() { return None; }
        let root = root.unwrap();

        let hit_record = HitRecord::new(
            &ray,
            root, 
            (ray.at(root) - self.center) / self.radius,  
        );

        Some(hit_record)
    }
}