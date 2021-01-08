use std::io::{Write, BufWriter};
use crate::image_formats::Image;


pub struct Png {
    data: Vec::<u8>, 
    width: usize, 
    height: usize, 
    bpp: usize
}

impl Png {
    pub fn new(width: usize, height: usize) -> Png {
        const BYTES_PER_PIXEL: usize = 3;

        let data: Vec<u8> = vec![0; width * height * BYTES_PER_PIXEL];

        Png {data, width, height, bpp: BYTES_PER_PIXEL}
    }
}

impl Image for Png {
    fn putpixel(&mut self, x: usize, y: usize, (r, g, b): (u8, u8, u8)) -> bool {
        let offset = self.bpp * (self.width() * y + x);
        if offset >= self.data.len() { return false; }

        self.data[offset] = r;
        self.data[offset + 1] = g;
        self.data[offset + 2] = b;

        true
    }

    fn write_image_data(&self, out: &mut impl Write) -> Result<usize, std::io::Error> {
        let w = BufWriter::new(out);
        let mut encoder = png::Encoder::new(
            w, 
            self.width as u32, 
            self.height as u32
        );

        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .expect("Failed to write PNG header");

        match writer.write_image_data(&self.data) {
            Ok(_) => Ok(0), 
            Err(e) => Err(e.into())
        }
    }

    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
}