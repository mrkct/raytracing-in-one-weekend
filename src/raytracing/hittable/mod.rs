use crate::{
    vec3::Vec3, 
    raytracing::{
        ray::Ray, 
        Material
    }
};
use std::rc::Rc;


pub mod sphere;

pub use {
    sphere::Sphere
};

pub struct HitRecord {
    pub p: Vec3, 
    pub normal: Vec3, 
    pub t: f64, 

    pub front_face: bool, 
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(material: Rc<dyn Material>, ray: &Ray, root: f64, outward_normal: Vec3) -> HitRecord {
        let front_face = Vec3::dot(ray.direction(), &outward_normal) < 0.; 
        HitRecord {
            material, 
            p: (ray.at(root)),  
            t: root, 
            front_face, 
            normal: if front_face { outward_normal } else { -outward_normal }
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub fn hits(hittable_objects: &[impl Hittable], ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest = t_max;
    let mut hit_record = None;

    for object in hittable_objects {
        if let Some(hr) = object.hit(ray, t_min, closest) {
            closest = hr.t;
            hit_record = Some(hr);
        }
    }

    hit_record
}
