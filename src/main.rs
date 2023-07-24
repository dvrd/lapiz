mod lib;

use lapiz::Canvas;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SIZE: usize = WIDTH * HEIGHT;
static mut PIXELS: [u32; SIZE] = [0; SIZE];

fn main() -> Result<(), ()> {
    let file_path = "o.ppm";

    unsafe {
        let mut canvas = Canvas::new(&mut PIXELS, WIDTH, HEIGHT);
        canvas.fill(0xff00ff00);
        canvas.save_to_ppm(file_path)?;
    };
    Ok(())
}
