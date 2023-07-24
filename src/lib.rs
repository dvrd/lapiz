use std::{fs::File, io::Write};

enum Color {
    Red,
    Green,
    Blue,
    Alpha,
}

pub struct Canvas {
    pixels: &'static mut [u32],
    width: usize,
    height: usize,
    size: usize,
}

impl Canvas {
    pub fn new(pixels: &'static mut [u32], width: usize, height: usize) -> Self {
        Canvas {
            pixels,
            width,
            height,
            size: width * height,
        }
    }

    pub fn fill(&mut self, color: u32) {
        for i in 0..self.size {
            self.pixels[i] = color;
        }
    }

    pub fn save_to_ppm(&self, path: &str) -> Result<(), ()> {
        let mut file = File::create(path).map_err(|msg| {
            eprintln!("Error: file {path} could not be open/created: {msg}");
        })?;

        let header = format!("P6\n{} {} 255\n", self.width, self.height);

        file.write_all(header.as_bytes()).map_err(|msg| {
            eprintln!("Error: could not write to file {path}: {msg}");
        })?;

        for i in 0..self.size {
            let pixel = self.pixels[i];
            let bytes: &[u8; 3] = &[
                extract_color(Color::Red, pixel),
                extract_color(Color::Green, pixel),
                extract_color(Color::Blue, pixel),
            ];

            file.write_all(bytes).map_err(|msg| {
                eprintln!("Error: could not write to file {path}: {msg}");
            })?;
        }

        Ok(())
    }
}

fn extract_u8_from_u32(pos: u32, byte: u32) -> u8 {
    ((byte >> 8 * pos) & 0xff) as u8
}

fn extract_color(color: Color, byte: u32) -> u8 {
    match color {
        Color::Red => extract_u8_from_u32(0, byte),
        Color::Green => extract_u8_from_u32(1, byte),
        Color::Blue => extract_u8_from_u32(2, byte),
        Color::Alpha => 0,
    }
}
