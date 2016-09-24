use core::fmt;

use spin::Mutex;

use board::uart;


pub static WRITER: Mutex<Writer> = Mutex::new(Writer);


pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            uart::write(c);
        }
        Ok(())
    }
}
