use std::fs::File;
use std::io::Write;
use std::io::Result;

pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(r:u8, g:u8, b:u8) -> Color {
        Color{
            r, g, b
        }
    }
}

pub struct PPM {
    width: u32,
    height: u32,
    data: Box<[u8]>
}

impl PPM{
    pub fn new(width: u32, height: u32) -> PPM {
        PPM {
            width,
            height,
            data: vec![0; (3 * width * height) as usize].into_boxed_slice()
        }
    }
/*
    fn size(&self) -> u32 {
        3 * self.width * self.height
    }

    fn get_offset(&self, x: u32, y:u32) -> u32 {
        let offset = (y * self.width + x) * 3;

        if offset >= self.size() {
            panic!("no.");
        }
        return offset;
    }

    pub fn set_pixel(&mut self, x:u32, y:u32, c: Color) {
        let o = self.get_offset(x, y) as usize;
        self.set_pixel_direct(o, c);
    }
*/
    pub fn set_pixel_direct(&mut self, o:usize, c:Color) {
        self.data[o] = c.r;
        self.data[o+1] = c.g;
        self.data[o+2] = c.b;
    }

    pub fn write_file(&self, name: &str) -> Result<()> {
        let mut file = File::create(name)?;
        file.write(format!("P6 {} {} 255\n", self.width, self.height).as_bytes())?;
        file.write(&self.data)?;
        Ok(())
    }
}