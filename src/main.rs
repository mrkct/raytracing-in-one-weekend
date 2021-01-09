mod raytracing;
mod vec3;
mod image_formats;

use std::fs::File;
use vec3::Vec3;
use std::rc::Rc;
use indicatif::{ProgressBar, ProgressStyle};
use image_formats::Image;
use raytracing::{
    material, 
    Material, 
    hittable
};

fn cool_picture_world() -> Vec<hittable::Sphere> {
    let mut world = vec![];

    let ground_material = Rc::new(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.push(hittable::Sphere::new(
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
                let sphere_material: Rc<dyn Material> = {
                    if choose_mat < 0.8 {
                        // Diffuse
                        Rc::from(material::Lambertian::new(rand_vec(-1., 1.) * rand_vec(-1., 1.)))
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = rand_vec(0.5, 1.0);
                        let fuzz = rand_val(0., 0.5);
                        Rc::from(material::Metal::new(albedo, fuzz))
                    } else {
                        // Glass
                        Rc::from(material::Dielectric::new(1.5))
                    }
                };
                world.push(hittable::Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let mat1 = Rc::new(material::Dielectric::new(1.5));
    world.push(hittable::Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Rc::new(material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.push(hittable::Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Rc::new(material::Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(hittable::Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3));

    world
}

fn main() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 300;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    const VERTICAL_FOV: f64 = 20.0;
    const APERTURE: f64 = 0.1;
    const DIST_TO_FOCUS: f64 = 10.0;

    const SAMPLES_PER_PIXEL: i32 = 50;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let camera = raytracing::Camera::new(
        &look_from, &look_at, &vup, 
        VERTICAL_FOV, ASPECT_RATIO, APERTURE, 
        DIST_TO_FOCUS
    );
    let world = cool_picture_world();
    let mut image = image_formats::png::Png::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let progressbar = ProgressBar::new(image.height() as u64);
    progressbar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} lines rendered ({eta})")
        .progress_chars("#>-")
    );

    raytracing::draw_world_with_callback(
        &camera, 
        &world, 
        &mut image, 
        SAMPLES_PER_PIXEL, 
        |row| {
            progressbar.set_position(row as u64);
        }
    );
    
    progressbar.finish_with_message("Writing to file...");

    let mut out = File::create("image.png").expect("Failed to create file");
    image.write_image_data(&mut out).expect("Failed to write to stdout");
    println!("Done!");
}