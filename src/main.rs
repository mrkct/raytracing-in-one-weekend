mod ppm;
mod vec3;
mod ray;
mod hittable;
mod camera;
mod material;

use std::fs::File;
use ppm::PPMImage;
use vec3::Vec3;
use ray::Ray;
use hittable::sphere::Sphere;
use std::rc::Rc;


pub fn clamp(min: f64, x: f64, max: f64) -> f64 {
    x.min(max).max(min)
}


fn ray_color(ray: &Ray, world: &Vec<impl hittable::Hittable>, max_depth: i32) -> Vec3 {
    if max_depth <= 0 { return Vec3::ZERO; }

    if let Some(hit_record) = hittable::hits(world, ray, 0.001, std::f64::INFINITY) {
        
        if let Some((attenuation, scattered)) = hit_record.material.as_ref().scatter(ray, &hit_record) {
            return attenuation * ray_color(&scattered, world, max_depth - 1);
        }

        return Vec3::ZERO;
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (1.0 + unit_direction.y);
    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
}

fn gamma2_color_correction(color: &Vec3) -> Vec3 {
    Vec3::new(
        color.x.sqrt(), 
        color.y.sqrt(), 
        color.z.sqrt()
    )
}

fn main() {
    
    const MAX_DEPTH: i32 = 10;

    // TODO: ASPECT RATIO is hardcoded inside Camera
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    

    let camera = camera::Camera::new();

    let material_ground = Rc::new(
        material::lambertian::Lambertian::new(Vec3::new(0.8, 0.8, 0.0))
    );
    let material_center = Rc::new(
        material::lambertian::Lambertian::new(Vec3::new(0.7, 0.3, 0.3))
    );
    let material_left = Rc::new(
        material::metal::Metal::new(Vec3::new(0.8, 0.8, 0.8))
    );
    let material_right = Rc::new(
        material::metal::Metal::new(Vec3::new(0.8, 0.6, 0.2))
    );

    let world = vec![ 
        Sphere::new(Vec3::new(0., -100.5, -1.), 100., material_ground.clone()), 
        Sphere::new(Vec3::new(0., 0., -1.), 0.5, material_center.clone()), 
        Sphere::new(Vec3::new(-1.0, 0., -1.), 0.5, material_left.clone()), 
        Sphere::new(Vec3::new(1.0, 0., -1.), 0.5, material_right.clone())
    ];

    let mut image = PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let samples_per_pixel = 40;
    let scale = 1.0 / samples_per_pixel as f64; // faster to multiply by this than dividing by samples_per_pixels

    for y in 0..image.height {
        println!("{} / {}", y, image.height);
        for x in 0..image.width {
            let j = image.height - y;
            
            let mut color = Vec3::ZERO;

            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rand::random::<f64>()) / (image.width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image.height - 1) as f64;
                let r = camera.get_ray(u, v);

                color += ray_color(&r, &world, MAX_DEPTH);
            }
            
            color.x = clamp(0., color.x * scale, 0.9999);
            color.y = clamp(0., color.y * scale, 0.9999);
            color.z = clamp(0., color.z * scale, 0.9999);
            
            image.putpixel(x, y, &gamma2_color_correction(&color));
        }
    }

    let mut out = File::create("image.ppm").expect("Failed to create file");
    image.write(&mut out).expect("Failed to write to stdout");
}
