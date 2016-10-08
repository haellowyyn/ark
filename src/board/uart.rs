//! Controls for the ARM PrimeCell UART (PL011).
//! ref: "ARM PrimeCell UART (PL011) Technical Reference Manual" [pl011-trm]

use mem::PAddr;
use super::UART0_BASE;

// UART registers [pl011-trm 3.2]
const UARTDR: PAddr = UART0_BASE + 0x000;   // data register
const UARTFR: PAddr = UART0_BASE + 0x018; // flag register

// UARTFR flags [pl011-trm 3.3.3]
const UARTFR_TXFF: u16 = 0b1 << 5;

pub fn write(c: u8) {
    unsafe {
        while *mmio_ptr!(UARTFR, u16) & UARTFR_TXFF != 0 {}
        *mmio_ptr!(UARTDR, u8) = c;
    }
}
