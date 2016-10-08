//! Driver for the ARM PrimeCell Color LCD Controller (PL110).
//! ref: "ARM PrimeCell Color LCD Controller (PL110) Technical Reference Manual" [pl110-trm]
//!
//! The PL110 implemented in the versatilepb differs slightly from the specification
//! in [pl110-trm]. See [versatilepb-ug 4.7.1].

use mem::PAddr;
use super::SYS_OSC4;
use super::CLCDC_BASE;

// CLCDC register offsets [pl110-trm 3.1]
// horizontal axis panel control register
const LCDTIM0: PAddr = CLCDC_BASE + 0x000;
// vertical axis panal control register
const LCDTIM1: PAddr = CLCDC_BASE + 0x004;
// clock and signal polarity control register
const LCDTIM2: PAddr = CLCDC_BASE + 0x008;
// upper panel frame base address register
const LCDUPBASE: PAddr = CLCDC_BASE + 0x010;
// control register
const LCDCTRL: PAddr = CLCDC_BASE + 0x018;

// LCDCTRL flags [pl110-trm 3.2.7]
const LCDEN: u32 = 0b1 << 0;        // LCD controller enable
const LCDBPP_24: u32 = 0b101 << 1;  // LCD bits per pixel: 24
const LCDTFT: u32 = 0b1 << 5;       // LCD if TFT
const LCDPWR: u32 = 0b1 << 11;      // LCD power enable


pub unsafe fn init(width: usize, height: usize, framebase: u32) {
    // Load timing registers depending on the resolution [pl110-trm 4.7.2].
    let (osc4, tim0, tim1, tim2) = match (width, height) {
        (240, 320) => (0x2c77, 0xc7a7bf38, 0x595b613f, 0x04ef1800),
        (320, 240) => (0x2c77, 0x9f7fbf4c, 0x818360ef, 0x053f1800),
        (176, 220) => (0x2c77, 0xf7c7bf28, 0x8b8d60db, 0x04af1800),
        (640, 480) => (0x2c77, 0x3f1f3f9c, 0x090b61df, 0x067f1800),
        (800, 600) => (0x2cac, 0x1313a4c4, 0x0505f657, 0x071f1800),
        _ => panic!("unsupported screen resolution"),
    };
    *mmio_ptr!(SYS_OSC4, u32) = osc4;
    *mmio_ptr!(LCDTIM0, u32) = tim0;
    *mmio_ptr!(LCDTIM1, u32) = tim1;
    *mmio_ptr!(LCDTIM2, u32) = tim2;

    // Load address of frame buffer.
    *mmio_ptr!(LCDUPBASE, u32) = framebase;
    // Set control flags.
    *mmio_ptr!(LCDCTRL, u32) = LCDEN | LCDBPP_24 | LCDTFT | LCDPWR;
}
