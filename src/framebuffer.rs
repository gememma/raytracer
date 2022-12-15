use std::{fs::File, io, io::BufWriter};

use png::{BitDepth, ColorType, Encoder, ScaledFloat};

use crate::colour::Colour;

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
    pub fn get_pixel(&self, x: usize, y: usize) -> Colour {
        self.buf[y * self.width + x].colour
    }
    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        self.buf[y * self.width + x].depth
    }

    pub fn add_caustics(&mut self, pmap: &FrameBuffer) -> FrameBuffer {
        // naively add caustics from a caustics frame buffer
        let mut new_fb = FrameBuffer::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let colour = if !pmap.get_pixel(x, y).is_nan() {
                    self.get_pixel(x, y) + pmap.get_pixel(x, y)
                } else {
                    self.get_pixel(x, y)
                };

                new_fb.plot_pixel(x, y, colour.r, colour.g, colour.b);
                new_fb.plot_depth(x, y, self.get_depth(x, y));
            }
        }
        new_fb
    }

    pub fn write_rgb_png(&self, filename: &str) -> io::Result<()> {
        // Open file
        let file = File::create(filename)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_source_gamma(ScaledFloat::new(1. / 2.));
        let mut writer = encoder.write_header().unwrap();

        let mut output = vec![];
        for p in &self.buf {
            output.push((255. * p.colour.r) as u8);
            output.push((255. * p.colour.g) as u8);
            output.push((255. * p.colour.b) as u8);
            output.push((255. * p.colour.a) as u8);
        }
        writer.write_image_data(&output).unwrap();
        Ok(())
    }

    pub fn write_depth_png(&self, filename: &str) -> io::Result<()> {
        // Open file
        let file = File::create(filename)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_source_gamma(ScaledFloat::new(1. / 2.));
        let mut writer = encoder.write_header().unwrap();

        let mut min: f32 = 0.;
        let mut max: f32 = 0.;

        // Calculate colour attenuation
        for p in &self.buf {
            min = min.min(p.depth);
            max = max.max(p.depth);
        }
        let diff = if max == min { 1. } else { max - min };

        let mut output = vec![];
        for p in &self.buf {
            for _ in 0..3 {
                output.push((255. * (p.depth - min) / diff) as u8);
            }
            output.push(255);
        }
        writer.write_image_data(&output).unwrap();
        Ok(())
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}
