use crate::colour::Colour;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    colour: Colour,
    depth: f32,
}

pub struct FrameBuffer {
    buf: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    // constructors
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            buf: vec![Pixel::default(); width * height],
            width,
            height,
        }
    }
    pub fn default() -> Self {
        Self::new(1024, 1024)
    }

    pub fn plot_pixel(&mut self, x: usize, y: usize, red: f32, green: f32, blue: f32) {
        self.buf[y * self.width + x].colour.r = red;
        self.buf[y * self.width + x].colour.g = green;
        self.buf[y * self.width + x].colour.b = blue;
    }
    pub fn plot_depth(&mut self, x: usize, y: usize, depth: f32) {
        self.buf[y * self.width + x].depth = depth;
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> (f32, f32, f32) {
        (
            self.buf[y * self.width + x].colour.r,
            self.buf[y * self.width + x].colour.g,
            self.buf[y * self.width + x].colour.b,
        )
    }
    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        self.buf[y * self.width + x].depth
    }
    pub fn write_rgb_file(&self, filename: &str) -> io::Result<()> {
        let mut min: f32 = 0.;
        let mut max: f32 = 0.;

        // Calculate colour attenuation
        for p in &self.buf {
            min = min.min(p.colour.r).min(p.colour.g).min(p.colour.b);
            max = max.max(p.colour.r).max(p.colour.g).max(p.colour.b);
        }
        let diff = if max == min { 1. } else { max - min };

        // Open file
        let mut file = File::create(filename)?;

        // Write out file
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        let mut output = vec![];
        for p in &self.buf {
            output.push((255. * (p.colour.r - min) / diff) as u8);
            output.push((255. * (p.colour.g - min) / diff) as u8);
            output.push((255. * (p.colour.b - min) / diff) as u8);
        }
        file.write_all(&output)
    }
    pub fn write_depth_file(&self, filename: &str) -> io::Result<()> {
        // Open file
        let mut file = File::create(filename)?;
        let mut min: f32 = 0.;
        let mut max: f32 = 0.;

        // Calculate colour attenuation
        for p in &self.buf {
            min = min.min(p.depth);
            max = max.max(p.depth);
        }
        let diff = if max == min { 1. } else { max - min };

        // Write out file
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        let mut output = vec![];
        for p in &self.buf {
            for _ in 0..3 {
                output.push((255. * (p.depth - min) / diff) as u8);
            }
        }
        file.write_all(&output)
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}
