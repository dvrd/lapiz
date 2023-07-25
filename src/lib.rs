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

    pub fn add_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for dy in 0..h {
            let y = y + dy;
            if y > 0 && self.height as i32 > y {
                for dx in 0..w {
                    let x = x + dx;
                    if x > 0 && self.width as i32 > x {
                        self.paint_pixel(x, y, color);
                    }
                }
            }
        }
    }

    pub fn add_circle(&mut self, cx: i32, cy: i32, r: usize, color: u32) {
        let x1 = cx - r as i32;
        let y1 = cy - r as i32;
        let x2 = cx + r as i32;
        let y2 = cy + r as i32;

        for y in y1..y2 {
            if y > 0 && self.height as i32 > y {
                for x in x1..x2 {
                    if x > 0 && self.width as i32 > x {
                        let dx = x - cx;
                        let dy = y - cy;
                        if dx.pow(2) + dy.pow(2) <= r.pow(2) as i32 {
                            self.paint_pixel(x, y, color);
                        }
                    }
                }
            }
        }
    }

    fn get_position(&self, x: i32, y: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    fn paint_pixel(&mut self, x: i32, y: i32, color: u32) {
        let position = self.get_position(x, y);
        self.pixels[position] = color;
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        if dx != 0 {
            let c = y1 - dy * x1 / dx;
            let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            for x in x1..=x2 {
                if x >= 0 && self.width as i32 > x {
                    let y = dy * x / dx + c;
                    let ny = dy * (x + 1) / dx + c;
                    let (y, ny) = if y > ny { (ny, y) } else { (y, ny) };
                    for y in y..=ny {
                        if y >= 0 && self.height as i32 > y {
                            self.paint_pixel(x, y, color);
                        }
                    }
                }
            }
        } else {
            let x = x1;
            if x >= 0 && self.width as i32 > x {
                let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
                for y in y1..y2 {
                    if y >= 0 && self.height as i32 > y {
                        self.paint_pixel(x, y, color);
                    }
                }
            }
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
