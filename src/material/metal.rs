use super::{Vec3, Ray, HitRecord, Material};
use crate::material;


pub struct Metal {
    albedo: Vec3, 
    fuzziness: f64
}

impl Metal {
    /*
        A metallic material that absorbs 'albedo' light in each direction 
        and applies a distortion to the rays with 'fuzziness'. Fuzziness 
        must be a value between [0, 1]
    */
    pub fn new(albedo: Vec3, fuzziness: f64) -> Metal {
        Metal {albedo, fuzziness: fuzziness.min(1.)}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = material::reflect(&ray_in.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + self.fuzziness * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction(), &hit_record.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}