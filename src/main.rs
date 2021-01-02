mod ppm;
mod vec3;
mod ray;
mod hittable;
mod camera;

use std::fs::File;
use ppm::PPMImage;
use vec3::Vec3;
use ray::Ray;
use hittable::sphere::Sphere;


pub fn clamp(min: f64, x: f64, max: f64) -> f64 {
    x.min(max).max(min)
}


fn ray_color(ray: &Ray, world: &Vec<impl hittable::Hittable>) -> Vec3 {
    if let Some(hit_record) = hittable::hits(world, ray, 0., std::f64::INFINITY) {
        return 0.5 * (hit_record.normal + Vec3::ONE);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (1.0 + unit_direction.y);
    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    // TODO: ASPECT RATIO is hardcoded inside Camera
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let camera = camera::Camera::new();
    let world = vec![
        Sphere::new(Vec3::new(0., 0., -1.), 0.5), 
        Sphere::new(Vec3::new(0., -100.5, -1.), 100.)
    ];

    let mut image = PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let samples_per_pixel = 100;
    let scale = 1.0 / samples_per_pixel as f64; // faster to multiply by this than dividing by samples_per_pixels

    for y in 0..image.height {
        for x in 0..image.width {
            let j = image.height - y;
            
            let mut color = Vec3::ZERO;

            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rand::random::<f64>()) / (image.width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image.height - 1) as f64;
                let r = camera.get_ray(u, v);

                color += ray_color(&r, &world);
            }
            
            color.x = clamp(0., color.x * scale, 0.9999);
            color.y = clamp(0., color.y * scale, 0.9999);
            color.z = clamp(0., color.z * scale, 0.9999);
            
            image.putpixel(x, y, &color);
        }
    }

    let mut out = File::create("image.ppm").expect("Failed to create file");
    image.write(&mut out).expect("Failed to write to stdout");
}
