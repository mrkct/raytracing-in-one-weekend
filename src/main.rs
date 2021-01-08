mod image_formats;
mod vec3;
mod ray;
mod hittable;
mod camera;
mod material;

use std::fs::File;
use vec3::Vec3;
use ray::Ray;
use hittable::sphere::Sphere;
use std::rc::Rc;

use indicatif::{ProgressBar, ProgressStyle};
use image_formats::Image;

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

fn cool_picture_world() -> Vec<Sphere> {
    let mut world = vec![];

    let ground_material = Rc::new(material::lambertian::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0), 
        1000.0, 
        ground_material.clone()
    ));

    let rand_val = |min, max| min + (max - min) * rand::random::<f64>();
    let rand_vec = |min, max| Vec3::new(rand_val(min, max), rand_val(min, max), rand_val(min, max));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_val(-1., 1.);
            let center = Vec3::new(a as f64 + 0.9 * rand_val(-1., 1.), 0.2, b as f64 + 0.9 * rand_val(-1., 1.));

            if (center - Vec3::new(4.0, 0.2, 0.)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material> = {
                    if choose_mat < 0.8 {
                        // Diffuse
                        Rc::from(material::lambertian::Lambertian::new(rand_vec(-1., 1.) * rand_vec(-1., 1.)))
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = rand_vec(0.5, 1.0);
                        let fuzz = rand_val(0., 0.5);
                        Rc::from(material::metal::Metal::new(albedo, fuzz))
                    } else {
                        // Glass
                        Rc::from(material::dielectric::Dielectric::new(1.5))
                    }
                };
                world.push(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let mat1 = Rc::new(material::dielectric::Dielectric::new(1.5));
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Rc::new(material::lambertian::Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Rc::new(material::metal::Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3));

    world
}

fn main() {
    
    const MAX_DEPTH: i32 = 10;

    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 300;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    const VERTICAL_FOV: f64 = 20.0;
    const APERTURE: f64 = 0.1;
    let dist_to_focus = 10.0;

    let camera = camera::Camera::new(
        &look_from, &look_at, &vup, 
        VERTICAL_FOV, ASPECT_RATIO, APERTURE, 
        dist_to_focus
    );

    let world = cool_picture_world();

    let mut image = image_formats::ppm::PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let samples_per_pixel = 50;
    let scale = 1.0 / samples_per_pixel as f64; // faster to multiply by this than dividing by samples_per_pixels

    let progressbar = ProgressBar::new(image.height() as u64);
    progressbar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} lines rendered ({eta})")
        .progress_chars("#>-")
    );

    for y in 0..image.height() {
        progressbar.set_position(y as u64);
        for x in 0..image.width() {
            let j = image.height() - y;
            
            let mut color = Vec3::ZERO;

            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rand::random::<f64>()) / (image.width() - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image.height() - 1) as f64;
                let r = camera.get_ray(u, v);

                color += ray_color(&r, &world, MAX_DEPTH);
            }
            
            color.x = clamp(0., color.x * scale, 0.9999);
            color.y = clamp(0., color.y * scale, 0.9999);
            color.z = clamp(0., color.z * scale, 0.9999);
            
            image.putpixel(x, y, gamma2_color_correction(&color).to_color());
        }
    }
    progressbar.finish_with_message("Writing to file...");

    let mut out = File::create("image.ppm").expect("Failed to create file");
    image.write_image_data(&mut out).expect("Failed to write to stdout");
    println!("Done!");
}
