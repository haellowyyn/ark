use uart;

pub fn main() -> ! {
    println(b"Entered user mode.");

    // Don't return.
    loop {}
}

fn println(string: &[u8]) {
    for c in string {
        uart::send(*c);
    }
    uart::send(b'\n');
}
