#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(custom_attribute)]
#![no_std]

use core::fmt;

extern crate rlibc;
extern crate spin;

#[macro_use]
mod io;
#[macro_use]
mod cpu;
mod board;
mod mem;
mod usermode;

#[no_mangle]
pub extern "C" fn kernel() -> ! {
    println!("Reached the kernel.");

    println!("Initializing MM...");
    unsafe { mem::init_mm() }
    println!("MM initialized.");

    loop {}

    // No entering user mode until we have set up the user mode translation tables.
    // enter_usermode();
}

// fn enter_usermode() -> ! {
//     unsafe {
//         set_sysreg!("SPSR_EL1", 0x0);
//         set_sysreg!("ELR_EL1", usermode::main as usize);
//         set_sysreg!("SP_EL0", reg!("sp"));
//         asm!("eret");
//     }
//
//     unreachable!();
// }


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
