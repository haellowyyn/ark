#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

use core::fmt;

use mem::info::*;

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

    println!("kernel loaded at: {:#x}-{:#x}", krnl_start(), krnl_end());
    println!(".text:   {:#x}-{:#x}", text_start(), text_end());
    println!(".rodata: {:#x}-{:#x}", rodata_start(), rodata_end());
    println!(".data:   {:#x}-{:#x}", data_start(), data_end());
    println!(".bss:    {:#x}-{:#x}", bss_start(), bss_end());
    println!("kernel stack: {:#x}-{:#x}", stack_bottom(), stack_top());

    mem::init_mm();

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
