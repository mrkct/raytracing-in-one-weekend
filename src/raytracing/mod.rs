use crate::{
    vec3::Vec3, 
    image_formats::Image
};

mod ray;
mod camera;
pub mod hittable;
pub mod material;

pub use {
    camera::Camera, 
    material::Material, 
    hittable::Hittable, 
    ray::Ray
};



fn clamp(min: f64, x: f64, max: f64) -> f64 {
    x.min(max).max(min)
}

fn gamma2_color_correction(color: &Vec3) -> Vec3 {
    Vec3::new(
        color.x.sqrt(), 
        color.y.sqrt(), 
        color.z.sqrt()
    )
}

pub fn draw_world_with_callback<F>(
    camera: &Camera, 
    world: &Vec<impl hittable::Hittable>, 
    image: &mut impl Image, 
    samples_per_pixel: i32, 
    on_row_render: F
) where F: Fn(usize) -> () {
    const MAX_DEPTH: i32 = 10;
    let scale = 1.0 / samples_per_pixel as f64;
    
    for y in 0..image.height() {
        for x in 0..image.width() {
            let j = image.height() - y;
            
            let mut color = Vec3::ZERO;

            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rand::random::<f64>()) / (image.width() - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image.height() - 1) as f64;
                let r = camera.get_ray(u, v);

                color += r.ray_color(&world, MAX_DEPTH);
            }
            
            color.x = clamp(0., color.x * scale, 0.9999);
            color.y = clamp(0., color.y * scale, 0.9999);
            color.z = clamp(0., color.z * scale, 0.9999);
            
            image.putpixel(x, y, gamma2_color_correction(&color).to_color());
        }
        on_row_render(y);
    }
}