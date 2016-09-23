use board::clcdc;

mod fb;
mod font;


const BGCOLOR: fb::Color = fb::Color::new(0x04, 0x2c, 0x36);
const FGCOLOR: fb::Color = fb::Color::new(0xa4, 0xb2, 0xb2);

const SYMBOL_WIDTH: usize = 8;
const SYMBOL_HEIGHT: usize = 15;
const NUM_SYM_PX: usize = SYMBOL_WIDTH * SYMBOL_HEIGHT;

static mut cursor: Cursor = Cursor { x: 0, y: 0 };

pub type Symbol = [u8; NUM_SYM_PX];


struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    fn set(&mut self, x: usize, y: usize) {
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
        if self.x + SYMBOL_WIDTH > fb::WIDTH {
            // line wrap
            self.x = 0;
            self.y += SYMBOL_HEIGHT;
        }
    }

    fn check_scroll(&mut self) {
        if self.y + SYMBOL_HEIGHT > fb::HEIGHT {
            scroll();
            self.y = fb::HEIGHT - SYMBOL_HEIGHT;
        }
    }
}


pub fn init() {
    unsafe { clcdc::init(fb::base() as u32) };
    clear();
}

pub fn write(chars: &[u8]) {
    for &c in chars {
        if c == b'\n' {
            unsafe { cursor.newline() }
            continue;
        }
        write_symbol(font::SYMBOLS[c as usize]);
    }
}

pub fn clear() {
    for y in 0..fb::HEIGHT {
        for x in 0..fb::WIDTH {
            fb::set_color(x, y, BGCOLOR);
        }
    }
    unsafe { cursor.set(0, 0) }
}

fn write_symbol(sym: &Symbol) {
    for i in 0..NUM_SYM_PX {
        let x = unsafe { cursor.x } + i % SYMBOL_WIDTH;
        let y = unsafe { cursor.y } + i / SYMBOL_WIDTH;
        fb::blend(x, y, FGCOLOR, sym[i]);
    }
    unsafe { cursor.fwd() }
}

fn scroll() {
    // Move screen contents up by one line.
    for y in 0..(fb::HEIGHT - SYMBOL_HEIGHT) {
        for x in 0..fb::WIDTH {
            let px_color = fb::get_color(x, y + SYMBOL_HEIGHT);
            fb::set_color(x, y, px_color);
        }
    }
    // Clear last line.
    for y in (fb::HEIGHT - SYMBOL_HEIGHT)..fb::HEIGHT {
        for x in 0..fb::WIDTH {
            fb::set_color(x, y, BGCOLOR);
        }
    }
}
