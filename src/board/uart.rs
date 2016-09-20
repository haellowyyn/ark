//! Controls for the ARM PrimeCell UART (PL011).
//! ref: "ARM PrimeCell UART (PL011) Technical Reference Manual" [pl011-trm]

use board::UART0_BASE;

// UART registers [pl011-trm 3.2]
const UARTDR: *mut u8 = (UART0_BASE + 0x000) as *mut u8;   // data register
const UARTFR: *mut u16 = (UART0_BASE + 0x018) as *mut u16; // flag register

// UARTFR flags [pl011-trm 3.3.3]
const UARTFR_TXFF: u16 = 0b1 << 5;

pub fn send(c: u8) {
    unsafe {
        while *UARTFR & UARTFR_TXFF != 0 {}
        *UARTDR = c;
    }
}
