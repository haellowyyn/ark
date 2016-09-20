//! Code specific to the 'versatilepb' board.
//! ref: "RealView Platform Baseboard for ARM926EJ-S User Guide" [virtualpb-ug]

pub mod uart;
pub mod clcdc;

// status and system control registers [virtualpb-ug 4.3]
const SYS_OSC4: *mut u32 = 0x1000001C as *mut u32;

// UART0 memory base address [virtualpb-ug 4.25]
const UART0_BASE: usize = 0x101f1000;

// CLCDC memory base address [virtualpb-ug 4.7]
const CLCDC_BASE: usize = 0x10120000;
