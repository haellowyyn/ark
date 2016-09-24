use core::fmt;
use core::fmt::Write;

mod screen;
mod serial;


/// Print line to screen and serial output.
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print to screen and serial output.
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}


pub fn _print(args: fmt::Arguments) {
    {
        let mut serial_writer = serial::WRITER.lock();
        serial_writer.write_fmt(args).unwrap();
    }
    {
        let mut screen_writer = screen::WRITER.lock();
        screen_writer.write_fmt(args).unwrap();
    }
}
