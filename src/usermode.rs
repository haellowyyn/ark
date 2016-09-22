use io;

pub fn main() -> ! {
    io::println(b"Entered user mode.");

    // Don't return.
    loop {}
}
