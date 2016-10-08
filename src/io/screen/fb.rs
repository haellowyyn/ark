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
        assert!(opacity <= 100);

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
        let &mut FrameBuf(ref mut buf) = self;
        buf[y][x] = color;
    }

    #[allow(needless_range_loop)]
    pub fn scroll(&mut self, dy: usize, fill: Color) {
        let &mut FrameBuf(ref mut buf) = self;
        for y in 0..(HEIGHT - dy) {
            for x in 0..WIDTH {
                buf[y][x] = buf[y + dy][x];
            }
        }
        for y in (HEIGHT - dy)..HEIGHT {
            for x in 0..WIDTH {
                buf[y][x] = fill;
            }
        }
    }
}
