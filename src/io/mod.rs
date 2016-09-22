mod console;
mod video;

use board::uart;

pub fn init_console() {
    console::init();
}

pub fn print(string: &[u8]) {
    for c in string {
        uart::send(*c);
    }
    console::write(string);
}

pub fn println(string: &[u8]) {
    print(string);
    print(b"\n");
}
