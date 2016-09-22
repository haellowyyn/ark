use color;
use color::Color;
use super::video;

const BGCOLOR: Color = 0x00362c04;
const FGCOLOR: Color = 0x00a2a294;

const SYMBOL_WIDTH: u32 = 10;
const SYMBOL_HEIGHT: u32 = 15;
const NUM_SYM_PX: usize = (SYMBOL_WIDTH * SYMBOL_HEIGHT) as usize;

static mut cursor: Cursor = Cursor { x: 0, y: 0 };

pub type Symbol = [u8; NUM_SYM_PX];

struct Cursor {
    x: u32,
    y: u32,
}

impl Cursor {
    fn set(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    fn fwd(&mut self) {
        self.x += SYMBOL_WIDTH;
        self.check_wrap();
        self.check_scroll()
    }

    fn newline(&mut self) {
        self.x = 0;
        self.y += SYMBOL_HEIGHT;
        self.check_scroll();
    }

    fn check_wrap(&mut self) {
        if self.x + SYMBOL_WIDTH > video::SCREEN_WIDTH {
            // line wrap
            self.x = 0;
            self.y += SYMBOL_HEIGHT;
        }
    }

    fn check_scroll(&mut self) {
        if self.y + SYMBOL_HEIGHT > video::SCREEN_HEIGHT {
            scroll();
            self.y = video::SCREEN_HEIGHT - SYMBOL_HEIGHT;
        }
    }
}


pub fn init() {
    video::init();
    clear();
}

pub fn write(chars: &[u8]) {
    for &c in chars {
        if c == b'\n' {
            unsafe { cursor.newline() }
            continue;
        }
        write_symbol(&[c % 100; NUM_SYM_PX]);
    }
}

pub fn clear() {
    for y in 0..video::SCREEN_HEIGHT {
        for x in 0..video::SCREEN_WIDTH {
            video::set_px(x, y, BGCOLOR);
        }
    }
    unsafe { cursor.set(0, 0) }
}

fn write_symbol(sym: &Symbol) {
    for i in 0..NUM_SYM_PX {
        let x = unsafe { cursor.x } + i as u32 % SYMBOL_WIDTH;
        let y = unsafe { cursor.y } + i as u32 / SYMBOL_WIDTH;
        video::set_px(x, y, color::blend(FGCOLOR, BGCOLOR, sym[i] as u32));
    }
    unsafe { cursor.fwd() }
}

fn scroll() {
    // Move screen contents up by one line.
    for y in 0..(video::SCREEN_HEIGHT - SYMBOL_HEIGHT) {
        for x in 0..video::SCREEN_WIDTH {
            let px_color = video::get_px(x, y + SYMBOL_HEIGHT);
            video::set_px(x, y, px_color);
        }
    }
    // Clear last line.
    for y in (video::SCREEN_HEIGHT - SYMBOL_HEIGHT)..video::SCREEN_HEIGHT {
        for x in 0..video::SCREEN_WIDTH {
            video::set_px(x, y, BGCOLOR);
        }
    }
}
