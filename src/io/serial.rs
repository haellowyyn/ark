use core::fmt;
use spin::Mutex;


pub static WRITER: Mutex<Writer> = Mutex::new(Writer);


pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        use board::uart;

        for b in s.bytes() {
            uart::write(b);
        }
        Ok(())
    }
}
