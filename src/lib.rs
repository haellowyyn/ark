#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;

#[macro_use]
mod macros;
mod board;
mod usermode;

use board::uart;

#[no_mangle]
pub extern "C" fn rust_main() {
    println(b"Booted to Rust.");
    unsafe { enter_usermode() }
}

unsafe fn enter_usermode() {
    set_sysreg!("SPSR_EL1", 0x0);
    set_sysreg!("ELR_EL1", usermode::main as usize);
    set_sysreg!("SP_EL0", 0x1000);
    asm!("eret");
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
