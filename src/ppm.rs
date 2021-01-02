use std::io::{Write, BufWriter};
use super::vec3::Vec3;


pub struct PPMImage {
    pub width: usize, 
    pub height: usize, 
    max_color_value: usize, 
    pub data: Vec::<u8>
}

impl PPMImage {
    pub fn new(width: usize, height: usize) -> PPMImage {
        PPMImage {
            max_color_value: 255, 
            width, height, data: vec![0; width*height*3]
        }
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        3 * (self.width * y + x)
    }

    /*
        Writes a single pixel on the image. Color is expected to be three 
        values between 0 and 1 (included)
    */
    pub fn putpixel(&mut self, x: usize, y: usize, color: &Vec3) -> bool {
        let offset = self.offset(x, y);
        if self.data.len() <= offset { return false; }
        
        let (r, g, b) = (
            (255.999 * color.x) as u8, 
            (255.999 * color.y) as u8, 
            (255.999 * color.z) as u8
        );

        self.data[offset] = r;
        self.data[offset+1] = g;
        self.data[offset+2] = b;
        true
    }

    pub fn write(&self, out: &mut impl Write) -> Result<usize, std::io::Error> {
        let mut stream = BufWriter::new(out);

        let mut written_bytes = 0;
        written_bytes += stream.write(b"P3\n")?;
        written_bytes += stream.write(self.width.to_string().as_bytes())?;
        written_bytes += stream.write(b" ")?;
        written_bytes += stream.write(self.height.to_string().as_bytes())?;
        written_bytes += stream.write(b"\n")?;
        written_bytes += stream.write(self.max_color_value.to_string().as_bytes())?;
        written_bytes += stream.write(b"\n")?;
        
        let mut i = 0;
        while i < self.data.len() {
            for _ in 0..3*(self.width-1) {
                written_bytes += stream.write(self.data[i].to_string().as_bytes())?;
                written_bytes += stream.write(b" ")?;
                i += 1;
            }

            // We need to write the last pixel separately since it has the \n at the end
            written_bytes += stream.write(self.data[i].to_string().as_bytes())?;
            written_bytes += stream.write(b" ")?;
            written_bytes += stream.write(self.data[i+1].to_string().as_bytes())?;
            written_bytes += stream.write(b" ")?;
            written_bytes += stream.write(self.data[i+2].to_string().as_bytes())?;
            written_bytes += stream.write(b"\n")?;
            
            i += 3;
        }

        Ok(written_bytes)
    }
}