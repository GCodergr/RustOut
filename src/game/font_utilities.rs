use sdl2::rect::Rect;
use crate::game::{WINDOW_WIDTH, WINDOW_HEIGHT};

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
pub fn _get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = calculate_font_scale(rect_width, rect_height, cons_width, cons_height, wr, hr);

    let cx = (WINDOW_WIDTH as i32 - w) / 2;
    let cy = (WINDOW_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn get_top_right_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect{
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = calculate_font_scale(rect_width, rect_height, cons_width, cons_height, wr, hr);

    let cx = WINDOW_WIDTH as i32 - w;
    let cy = 0;
    rect!(cx, cy, w, h)
}

pub fn get_top_left_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect{
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = calculate_font_scale(rect_width, rect_height, cons_width, cons_height, wr, hr);

    let cx = 0;
    let cy = 0;
    rect!(cx, cy, w, h)
}

fn calculate_font_scale(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32, wr: f32, hr: f32) -> (i32, i32) {
    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };
    (w, h)
}