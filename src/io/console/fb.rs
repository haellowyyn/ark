pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

static mut FRAMEBUF: [[Color; WIDTH]; HEIGHT] = [[Color::blank(); WIDTH]; HEIGHT];


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    _p: u8, // padding
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            red: r,
            green: g,
            blue: b,
            _p: 0,
        }
    }

    const fn blank() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            _p: 0,
        }
    }
}


pub fn set_color(x: usize, y: usize, color: Color) {
    // TODO bounds check

    unsafe { FRAMEBUF[y][x] = color }
}

pub fn get_color(x: usize, y: usize) -> Color {
    // TODO bounds check

    unsafe { FRAMEBUF[y][x] }
}

pub fn blend(x: usize, y: usize, color: Color, opacity: u8) {
    // TODO check opacity <= 100

    let (fgr, fgg, fgb) = (color.red as u32, color.green as u32, color.blue as u32);
    let fgopa = opacity as u32;

    let bgcolor = get_color(x, y);
    let (bgr, bgg, bgb) = (bgcolor.red as u32, bgcolor.green as u32, bgcolor.blue as u32);
    let bgopa = 100 - fgopa;

    let mixed = Color::new(((fgr * fgopa + bgr * bgopa) / 100) as u8,
                           ((fgg * fgopa + bgg * bgopa) / 100) as u8,
                           ((fgb * fgopa + bgb * bgopa) / 100) as u8);
    set_color(x, y, mixed);
}

pub fn base() -> usize {
    unsafe { &FRAMEBUF as *const _ as usize }
}
