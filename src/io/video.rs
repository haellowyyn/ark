use board::clcdc;
use color::Color;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
const NUM_PX: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

static mut framebuf: [Color; NUM_PX] = [0; NUM_PX];


pub fn init() {
    unsafe {
        let framebase = (&framebuf as *const [u32; NUM_PX]) as u32;
        clcdc::init(framebase);
    }
}

pub fn set_px(x: u32, y: u32, color: Color) {
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        // TODO return error
    }
    let fb_idx = (y * SCREEN_WIDTH + x) as usize;
    unsafe { framebuf[fb_idx] = color }
}

pub fn get_px(x: u32, y: u32) -> Color {
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        // TODO return error
    }
    let fb_idx = (y * SCREEN_WIDTH + x) as usize;
    unsafe { framebuf[fb_idx] }
}
