#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println(b"Hello, World!");

    // Don't return.
    loop {}
}

// This is specific to the 'versatilepb' machine.
// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0224i/Chdbeibh.html
const UART0: *mut u8 = 0x101f1000 as *mut _;

fn println(string: &[u8]) {
    for c in string {
        unsafe { *UART0 = *c };
    }
    unsafe { *UART0 = b'\n' };
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
