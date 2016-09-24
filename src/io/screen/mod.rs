use core::fmt;
use spin::Mutex;

use self::fb::FrameBuf;

mod fb;
mod font;


const DEFAULT_FGCOLOR: fb::Color = fb::Color::new(0xa4, 0xb2, 0xb2);
const DEFAULT_BGCOLOR: fb::Color = fb::Color::new(0x04, 0x2c, 0x36);

const SYMBOL_WIDTH: usize = 8;
const SYMBOL_HEIGHT: usize = 15;
const NUM_SYM_PX: usize = SYMBOL_WIDTH * SYMBOL_HEIGHT;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer::new());


pub type Symbol = [u8; NUM_SYM_PX];


pub struct Writer {
    x_pos: usize,
    fgcolor: fb::Color,
    bgcolor: fb::Color,
    fb: FrameBuf,
    initialized: bool,
}

impl Writer {
    const fn new() -> Self {
        Writer {
            initialized: false,
            x_pos: 0,
            fgcolor: DEFAULT_FGCOLOR,
            bgcolor: DEFAULT_BGCOLOR,
            fb: FrameBuf::new([[DEFAULT_BGCOLOR; fb::WIDTH]; fb::HEIGHT]),
        }
    }

    fn init(&mut self) {
        use board::clcdc;
        unsafe { clcdc::init(&self.fb as *const _ as u32) };
    }

    fn write_byte(&mut self, b: u8) {
        if !self.initialized {
            self.init();
        }

        match b {
            b'\n' => self.newline(),
            _ => {
                self.write_symbol(font::SYMBOLS[b as usize]);
                self.advance();
            }
        }
    }

    fn write_symbol(&mut self, symbol: &Symbol) {
        for (i, &opacity) in symbol.iter().enumerate() {
            let x = self.x_pos + i % SYMBOL_WIDTH;
            let y = fb::HEIGHT - SYMBOL_HEIGHT + i / SYMBOL_WIDTH;
            let c = self.bgcolor.blend(self.fgcolor, opacity);
            self.fb.set(x, y, c);
        }
    }

    fn advance(&mut self) {
        self.x_pos += SYMBOL_WIDTH;
        if self.x_pos + SYMBOL_WIDTH > fb::WIDTH {
            self.newline();
        }
    }

    fn newline(&mut self) {
        self.fb.scroll(SYMBOL_HEIGHT, self.bgcolor);
        self.x_pos = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }
}


// struct Cursor {
//     x: usize,
//     y: usize,
// }
//
// impl Cursor {
//     fn set(&mut self, x: usize, y: usize) {
//         self.x = x;
//         self.y = y;
//     }
//
//     fn fwd(&mut self) {
//         self.x += SYMBOL_WIDTH;
//         self.check_wrap();
//         self.check_scroll()
//     }
//
//     fn newline(&mut self) {
//         self.x = 0;
//         self.y += SYMBOL_HEIGHT;
//         self.check_scroll();
//     }
//
//     fn check_wrap(&mut self) {
//         if self.x + SYMBOL_WIDTH > fb::WIDTH {
//             // line wrap
//             self.x = 0;
//             self.y += SYMBOL_HEIGHT;
//         }
//     }
//
//     fn check_scroll(&mut self) {
//         if self.y + SYMBOL_HEIGHT > fb::HEIGHT {
//             scroll();
//             self.y = fb::HEIGHT - SYMBOL_HEIGHT;
//         }
//     }
// }


// pub fn write(chars: &[u8]) {
//     for &c in chars {
//         if c == b'\n' {
//             unsafe { cursor.newline() }
//             continue;
//         }
//         write_symbol(font::SYMBOLS[c as usize]);
//     }
// }
//
// pub fn clear() {
//     for y in 0..fb::HEIGHT {
//         for x in 0..fb::WIDTH {
//             fb::set_color(x, y, BGCOLOR);
//         }
//     }
//     unsafe { cursor.set(0, 0) }
// }
//
// fn write_symbol(sym: &Symbol) {
//     for i in 0..NUM_SYM_PX {
//         let x = unsafe { cursor.x } + i % SYMBOL_WIDTH;
//         let y = unsafe { cursor.y } + i / SYMBOL_WIDTH;
//         fb::blend(x, y, FGCOLOR, sym[i]);
//     }
//     unsafe { cursor.fwd() }
// }
//
// fn scroll() {
//     // Move screen contents up by one line.
//     for y in 0..(fb::HEIGHT - SYMBOL_HEIGHT) {
//         for x in 0..fb::WIDTH {
//             let px_color = fb::get_color(x, y + SYMBOL_HEIGHT);
//             fb::set_color(x, y, px_color);
//         }
//     }
//     // Clear last line.
//     for y in (fb::HEIGHT - SYMBOL_HEIGHT)..fb::HEIGHT {
//         for x in 0..fb::WIDTH {
//             fb::set_color(x, y, BGCOLOR);
//         }
//     }
// }
