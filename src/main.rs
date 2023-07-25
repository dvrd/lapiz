use lapiz::Canvas;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const COLS: usize = 8 * 2;
const ROWS: usize = 6 * 2;
const CELL_WIDTH: usize = WIDTH / COLS;
const CELL_HEIGTH: usize = HEIGHT / ROWS;
const SIZE: usize = WIDTH * HEIGHT;

static mut PIXELS: [u32; SIZE] = [0; SIZE];

const RED: u32 = 0xff2020ff;
const GREEN: u32 = 0xff00ff00;
const BLUE: u32 = 0xffff0000;
const GREY: u32 = 0xff202020;
// const WHITE: u32 = 0xffffffff;

fn main() -> Result<(), ()> {
    checkers_example()?;
    Ok(())
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Examples
#[allow(dead_code)]
fn line_example() -> Result<(), ()> {
    let file_path = "lines.ppm";

    unsafe {
        let mut canvas = Canvas::new(&mut PIXELS, WIDTH, HEIGHT);
        canvas.fill(GREY);
        canvas.draw_line(0, 0, WIDTH as i32, HEIGHT as i32, RED);
        canvas.draw_line(WIDTH as i32, 0, 0, HEIGHT as i32, RED);

        canvas.draw_line(0, 0, WIDTH as i32 / 4, HEIGHT as i32, GREEN);
        canvas.draw_line(WIDTH as i32 / 4, 0, 0, HEIGHT as i32, GREEN);

        canvas.draw_line(WIDTH as i32, 0, WIDTH as i32 / 4 * 3, HEIGHT as i32, GREEN);
        canvas.draw_line(WIDTH as i32 / 4 * 3, 0, WIDTH as i32, HEIGHT as i32, GREEN);

        canvas.draw_line(0, HEIGHT as i32 / 2, WIDTH as i32, HEIGHT as i32 / 2, BLUE);
        canvas.draw_line(WIDTH as i32 / 2, 0, WIDTH as i32 / 2, HEIGHT as i32, BLUE);
        // for y in 0..(ROWS * 2) {
        //     for x in 0..(COLS * 2) {
        //         let u = x as f32 / COLS as f32;
        //         let v = y as f32 / ROWS as f32;
        //         let t = (u + v) / 2.;

        //         let circle_r = if CELL_HEIGTH < CELL_WIDTH {
        //             CELL_HEIGTH
        //         } else {
        //             CELL_WIDTH
        //         };
        //         let x = x * CELL_WIDTH + CELL_WIDTH / 2;
        //         let y = y * CELL_HEIGTH + CELL_HEIGTH / 2;
        //         let circle_r = lerp(circle_r as f32 / 4., circle_r as f32, t) as usize;
        //         canvas.add_circle(x as i32, y as i32, circle_r / 2, RED);
        //     }
        // }
        canvas.save_to_ppm(file_path)?;
    };
    Ok(())
}

#[allow(dead_code)]
fn circle_example() -> Result<(), ()> {
    let file_path = "circle.ppm";

    unsafe {
        let mut canvas = Canvas::new(&mut PIXELS, WIDTH, HEIGHT);
        canvas.fill(GREY);
        for y in 0..(ROWS) {
            for x in 0..(COLS) {
                let u = x as f32 / COLS as f32;
                let v = y as f32 / ROWS as f32;
                let t = (u + v) / 2.;

                let circle_r = if CELL_HEIGTH < CELL_WIDTH {
                    CELL_HEIGTH
                } else {
                    CELL_WIDTH
                };
                let x = x * CELL_WIDTH + CELL_WIDTH / 2;
                let y = y * CELL_HEIGTH + CELL_HEIGTH / 2;
                let circle_r = lerp(circle_r as f32 / 4., circle_r as f32, t) as usize;
                canvas.add_circle(x as i32, y as i32, circle_r / 2, RED);
            }
        }
        canvas.save_to_ppm(file_path)?;
    };
    Ok(())
}

#[allow(dead_code)]
fn checkers_example() -> Result<(), ()> {
    let file_path = "checkers.ppm";
    let rect_w = 50 * 4;
    let rect_h = 30 * 4;

    unsafe {
        let mut canvas = Canvas::new(&mut PIXELS, WIDTH, HEIGHT);
        canvas.fill(GREY);
        for y in 0..ROWS {
            for x in 0..COLS {
                let color = if (x + y) % 2 == 0 { RED } else { GREY };
                let x = x * CELL_WIDTH;
                let y = y * CELL_HEIGTH;
                canvas.add_rect(x as i32, y as i32, rect_w, rect_h, color);
            }
        }
        canvas.save_to_ppm(file_path)?;
    };
    Ok(())
}
