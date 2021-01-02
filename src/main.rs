mod ppm;
mod vec3;
mod ray;
mod hittable;

use std::fs::File;
use ppm::PPMImage;
use vec3::Vec3;
use ray::Ray;
use hittable::sphere::Sphere;



fn ray_color(ray: &Ray, world: &Vec<impl hittable::Hittable>) -> Vec3 {
    if let Some(hit_record) = hittable::hits(world, ray, 0., std::f64::INFINITY) {
        return 0.5 * (hit_record.normal + Vec3::ONE);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (1.0 + unit_direction.y);
    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera stuff
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::with_z(Vec3::ZERO, focal_length);

    // World
    let world = vec![
        Sphere::new(Vec3::new(0., 0., -1.), 0.5), 
        Sphere::new(Vec3::new(0., -100.5, -1.), 100.)
    ];

    let mut image = PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for y in 0..image.height {
        for x in 0..image.width {
            let j = image.height - y;
            
            let u = x as f64 / (image.width - 1) as f64;
            let v = j as f64 / (image.height - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            image.putpixel(x, y, &ray_color(&r, &world));
        }
    }

    let mut out = File::create("image.ppm").expect("Failed to create file");
    image.write(&mut out).expect("Failed to write to stdout");
}
