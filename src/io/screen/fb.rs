pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;


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

    pub fn blend(&self, other: Color, opacity: u8) -> Self {
        // TODO check opacity <= 100

        let (fgr, fgg, fgb) = (other.red as u32, other.green as u32, other.blue as u32);
        let fgopa = opacity as u32;

        let (bgr, bgg, bgb) = (self.red as u32, self.green as u32, self.blue as u32);
        let bgopa = 100 - fgopa;

        Color::new(((fgr * fgopa + bgr * bgopa) / 100) as u8,
                   ((fgg * fgopa + bgg * bgopa) / 100) as u8,
                   ((fgb * fgopa + bgb * bgopa) / 100) as u8)
    }
}


pub struct FrameBuf([[Color; WIDTH]; HEIGHT]);

impl FrameBuf {
    pub const fn new(buf: [[Color; WIDTH]; HEIGHT]) -> Self {
        FrameBuf(buf)
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        // TODO bounds check
        let &mut FrameBuf(ref mut buf) = self;
        buf[y][x] = color;
    }

    pub fn scroll(&mut self, dy: usize, fill: Color) {
        // TODO check dy < HEIGHT
        let &mut FrameBuf(ref mut buf) = self;
        for y in 0..HEIGHT {
            buf[y] = if y < HEIGHT - dy {
                buf[y + dy]
            } else {
                [fill; WIDTH]
            };
        }
    }
}
