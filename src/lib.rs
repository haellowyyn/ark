#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

use core::fmt;

extern crate rlibc;
extern crate spin;

extern "C" {
    static _krnl_start: u32;
    static _krnl_end: u32;
    static _text_start: u32;
    static _text_end: u32;
    static _rodata_start: u32;
    static _rodata_end: u32;
    static _data_start: u32;
    static _data_end: u32;
    static _bss_start: u32;
    static _bss_end: u32;
    static _stack_bottom: u32;
    static _stack_top: u32;
}

#[macro_use]
mod io;
#[macro_use]
mod cpu;
mod board;
mod usermode;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Booted to Rust.");

    unsafe {
        println!("kernel loaded at: {:#p}-{:#p}", &_krnl_start, &_krnl_end);
        println!(".text:   {:#p}-{:#p}", &_text_start, &_text_end);
        println!(".rodata: {:#p}-{:#p}", &_rodata_start, &_rodata_end);
        println!(".data:   {:#p}-{:#p}", &_data_start, &_data_end);
        println!(".bss:    {:#p}-{:#p}", &_bss_start, &_bss_end);
        println!("kernel stack: {:#p}-{:#p}", &_stack_bottom, &_stack_top);
    }

    enter_usermode();
}

fn enter_usermode() -> ! {
    unsafe {
        set_sysreg!("SPSR_EL1", 0x0);
        set_sysreg!("ELR_EL1", usermode::main as usize);
        set_sysreg!("SP_EL0", reg!("sp"));
        asm!("eret");
    }

    unreachable!();
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
