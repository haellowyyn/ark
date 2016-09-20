//! Controls for the ARM PrimeCell UART (PL011).
//! ref: "ARM PrimeCell UART (PL011) Technical Reference Manual"

use board::UART0;

const UARTDR: usize = 0x000;
const UARTFR: usize = 0x018;

const UARTFR_TXFF: u16 = 1 << 5;

pub fn send(c: u8) {
    let dr = (UART0 + UARTDR) as *mut u8;
    let fr = (UART0 + UARTFR) as *mut u16;

    unsafe {
        while *fr & UARTFR_TXFF != 0 {}
        *dr = c;
    }
}
