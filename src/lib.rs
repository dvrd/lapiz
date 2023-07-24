use std::{fs::File, io::Write};

enum Color {
    Red,
    Green,
    Blue,
    Alpha,
}

pub fn fill(pixels: &mut [u32], width: usize, height: usize, color: u32) {
    for i in 0..width * height {
        pixels[i] = color;
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

pub fn save_to_ppm(pixels: &mut [u32], width: usize, height: usize, path: &str) -> Result<(), ()> {
    let mut file = File::create(path).map_err(|msg| {
        eprintln!("Error: file {path} could not be open/created: {msg}");
    })?;

    let header = format!("P6\n{} {} 255\n", width, height);

    file.write_all(header.as_bytes()).map_err(|msg| {
        eprintln!("Error: could not write to file {path}: {msg}");
    })?;

    for i in 0..width * height {
        let pixel = pixels[i];
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
