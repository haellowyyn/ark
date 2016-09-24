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

        let framebase = &self.fb as *const _ as u32;
        unsafe { clcdc::init(fb::WIDTH, fb::HEIGHT, framebase) };
    }

    fn write_byte(&mut self, b: u8) {
        if !self.initialized {
            self.init();
        }

        match b {
            b'\n' => self.newline(),
            _ => {
                let symbol = match font::SYMBOLS[b as usize] {
                    Some(sym) => sym,
                    None => font::NONPRINTABLE,
                };
                self.write_symbol(symbol);
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
