#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

use core::fmt;

extern crate rlibc;
extern crate spin;

#[macro_use]
mod io;
#[macro_use]
mod cpu;
mod board;
mod usermode;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Booted to Rust.");

    let (_start, _end): (usize, usize);
    unsafe { asm!("ldr $0, =_start; ldr $1, =_end" : "=r"(_start), "=r"(_end)) }
    println!("Kernel loaded at {:#x} - {:#x}", _start, _end);

    enter_usermode();

    panic!("unreachable");
}

fn enter_usermode() {
    unsafe {
        set_sysreg!("SPSR_EL1", 0x0);
        set_sysreg!("ELR_EL1", usermode::main as usize);
        set_sysreg!("SP_EL0", reg!("sp"));
        asm!("eret");
    }
}


/// Panic handler.
#[lang = "panic_fmt"]
extern "C" fn panic_fmt(fmt: fmt::Arguments, file: &str, line: u32) -> ! {
    println!("!! PANIC at {}:{}: \"{}\"", file, line, fmt);

    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
