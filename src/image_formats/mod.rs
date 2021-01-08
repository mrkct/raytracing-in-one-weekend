pub mod ppm;
pub mod png;

use std::io::Write;



pub trait Image {
    fn putpixel(&mut self, x: usize, y: usize, color: (u8, u8, u8)) -> bool;
    fn write_image_data(&self, out: &mut impl Write) -> Result<usize, std::io::Error>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;
}