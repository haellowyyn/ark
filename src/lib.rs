#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod macros;
mod board;
#[macro_use]
mod io;
mod usermode;

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Booted to Rust.");

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
    println!("panic!");
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
