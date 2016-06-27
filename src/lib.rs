#![feature(lang_items)]
#![no_std]

extern crate rlibc;

mod uart;
mod versatilepb;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println(b"Hello, World!");

    // Don't return.
    loop {}
}


fn println(string: &[u8]) {
    for c in string {
        uart::send(*c);
    }
    uart::send(b'\n');
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
