use crate::{
    vec3::Vec3, 
    image_formats::Image
};
use rayon::prelude::*;


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
use std::sync::Arc;


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
    world: &Vec<Box<dyn Hittable + Send + Sync>>, 
    image: &mut impl Image, 
    samples_per_pixel: i32, 
    on_row_render: F
) 
where 
    F: Fn() -> () + Send + Sync + 'static
{
    const MAX_DEPTH: i32 = 10;
    let scale = 1.0 / samples_per_pixel as f64;
    
    let image_width = image.width();
    let image_height = image.height();

    let on_row_render = Arc::new(on_row_render);

    let pixels = (0..image.height())
        .into_par_iter()
        .map_with(
            on_row_render.clone(), 
            |on_row_render, y| {
                let row = (0..image_width).into_par_iter().map(|x| {
                let j = image_height - y;

                let mut color = (0..samples_per_pixel).into_par_iter().map(|_| {
                    let u = (x as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                    let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                    let r = camera.get_ray(u, v);

                    r.ray_color(world, MAX_DEPTH)
                }).reduce(|| Vec3::ZERO, |a, b| a + b);

                color.x = clamp(0., color.x * scale, 0.9999);
                color.y = clamp(0., color.y * scale, 0.9999);
                color.z = clamp(0., color.z * scale, 0.9999);

                gamma2_color_correction(&color).to_color()
            }
        ).collect::<Vec<_>>();
        on_row_render();
        row
    }).collect::<Vec<_>>();

    for (y, row_pixels) in pixels.iter().enumerate() {
        for (x, color) in row_pixels.iter().enumerate() {
            image.putpixel(x, y, *color);
        }
    }
}