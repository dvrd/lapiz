use lapiz::{fill, save_to_ppm};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SIZE: usize = WIDTH * HEIGHT;
static mut PIXELS: [u32; SIZE] = [0; SIZE];

fn main() -> Result<(), ()> {
    let file_path = "o.ppm";

    unsafe {
        fill(&mut PIXELS, WIDTH, HEIGHT, 0xff00ff00);
        save_to_ppm(&mut PIXELS, WIDTH, HEIGHT, file_path)?;
    };
    Ok(())
}
