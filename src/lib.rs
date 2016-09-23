#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate rlibc;

#[macro_use]
mod macros;

mod board;
mod io;
mod usermode;

#[no_mangle]
pub extern "C" fn rust_main() {
    io::println(b"Booted to Rust.");

    io::println(b"Initializing console...");
    io::init_console();

    unsafe { enter_usermode() }
}

unsafe fn enter_usermode() {
    set_sysreg!("SPSR_EL1", 0x0);
    set_sysreg!("ELR_EL1", usermode::main as usize);
    set_sysreg!("SP_EL0", 0x1000);
    asm!("eret");
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    // TODO print something useful and halt
    io::println(b"panic!");
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
