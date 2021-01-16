mod raytracing;
mod vec3;
mod image_formats;

use std::fs::File;
use vec3::Vec3;
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use image_formats::Image;
use raytracing::{
    material, 
    Material, 
    hittable
};
use clap::Clap;


fn cool_picture_world() -> Vec<Box<dyn hittable::Hittable + Send + Sync>> {
    let mut world: Vec<Box<dyn hittable::Hittable + Send + Sync>> = vec![];

    let ground_material: Arc<Box<(dyn Material + Send + Sync + 'static)>> = Arc::new(Box::new(
        material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5))
    ));
    world.push(Box::new(hittable::Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0), 
        1000.0, 
        ground_material.clone()
    )));

    let rand_val = |min, max| min + (max - min) * rand::random::<f64>();
    let rand_vec = |min, max| Vec3::new(rand_val(min, max), rand_val(min, max), rand_val(min, max));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_val(-1., 1.);
            let center = Vec3::new(a as f64 + 0.9 * rand_val(-1., 1.), 0.2, b as f64 + 0.9 * rand_val(-1., 1.));

            if (center - Vec3::new(4.0, 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<Box<dyn Material + Send + Sync>> = {
                    if choose_mat < 0.8 {
                        // Diffuse
                        Arc::new(Box::new(material::Lambertian::new(rand_vec(-1., 1.) * rand_vec(-1., 1.))))
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = rand_vec(0.5, 1.0);
                        let fuzz = rand_val(0., 0.5);
                        Arc::new(Box::new(material::Metal::new(albedo, fuzz)))
                    } else {
                        // Glass
                        Arc::new(Box::new(material::Dielectric::new(1.5)))
                    }
                };
                world.push(Box::new(hittable::Sphere::new(
                    center, 0.2, sphere_material
                )));
            }
        }
    }

    let mat1: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        material::Dielectric::new(1.5)
    ));
    world.push(Box::new(hittable::Sphere::new(
        Vec3::new(0.0, 1.0, 0.0), 1.0, mat1
    )));

    let mat2: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1))
    ));
    world.push(Box::new(hittable::Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2
    )));

    let mat3: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        material::Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)
    ));
    world.push(Box::new(
        hittable::Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3
    )));

    world
}

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Marco C. <marco.cutecchia@outlook.it>")]
struct Options {
    #[clap(short, long, default_value="image.png")]
    output_name: String, 
    #[clap(short, long, default_value="640")]
    width: usize, 
    #[clap(short, long, default_value="480")]
    height: usize, 
    #[clap(short, long, default_value="50")]
    samples_per_pixels: i32, 
    #[clap(short, long, default_value="20.0")]
    vertical_fov: f64
}

fn main() { 
    const APERTURE: f64 = 0.1;
    const DIST_TO_FOCUS: f64 = 10.0;

    let opt = Options::parse();
    println!("{:?}", opt);

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let aspect_ratio = opt.width as f64 / opt.height as f64;

    let camera = raytracing::Camera::new(
        &look_from, &look_at, &vup, 
        opt.vertical_fov, 
        aspect_ratio, 
        APERTURE, 
        DIST_TO_FOCUS
    );
    let world = cool_picture_world();
    let mut image = image_formats::png::Png::new(opt.width, opt.height);

    let progressbar = Box::new(ProgressBar::new(image.height() as u64));
    progressbar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} lines rendered ({eta})")
        .progress_chars("#>-")
    );

    let position = Box::new(std::sync::Mutex::new(0));
    raytracing::draw_world_with_callback(
        &camera, 
        &world, 
        &mut image, 
        opt.samples_per_pixels, 
        move || {
            let mut p = position.lock().unwrap();
            *p += 1;
            progressbar.set_position(*p);
        }
    );

    let mut out = File::create(opt.output_name).expect("Failed to create file");
    image.write_image_data(&mut out).expect("Failed to write to stdout");
    println!("Done!");
}