use super::{Vec3, Ray, HitRecord, Material};


pub struct Metal {
    albedo: Vec3
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2. * Vec3::dot(v, n) * n
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal {albedo}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray_in.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction(), &hit_record.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}