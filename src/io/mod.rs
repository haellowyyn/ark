mod console;
mod serial;

use core::fmt;
use core::fmt::Write;


/// Print line to screen and serial output.
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print to screen and serial output.
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}


pub fn init() {
    console::init();
}

pub fn _print(args: fmt::Arguments) {
    let mut serial_writer = serial::WRITER.lock();
    serial_writer.write_fmt(args).unwrap();
}
