//! Driver for the ARM PrimeCell Color LCD Controller (PL110).
//! ref: "ARM PrimeCell Color LCD Controller (PL110) Technical Reference Manual" [pl110-trm]
//!
//! The PL110 implemented in the versatilepb differs slightly from the specification
//! in [pl110-trm]. See [versatilepb-ug 4.7.1].

use board::SYS_OSC4;
use board::CLCDC_BASE;

// CLCDC register offsets [pl110-trm 3.1]
const LCDTIM0: *mut u32 = (CLCDC_BASE + 0x000) as *mut u32;    // horizontal axis panel control register
const LCDTIM1: *mut u32 = (CLCDC_BASE + 0x004) as *mut u32;    // vertical axis panal control register
const LCDTIM2: *mut u32 = (CLCDC_BASE + 0x008) as *mut u32;    // clock and signal polarity control register
const LCDUPBASE: *mut u32 = (CLCDC_BASE + 0x010) as *mut u32;  // upper panel frame base address register
const LCDCTRL: *mut u32 = (CLCDC_BASE + 0x018) as *mut u32;    // control register

// LCDCTRL flags [pl110-trm 3.2.7]
const LCDEN: u32 = 0b1 << 0;        // LCD controller enable
const LCDBPP_24: u32 = 0b101 << 1;  // LCD bits per pixel: 24
const LCDTFT: u32 = 0b1 << 5;       // LCD if TFT
const LCDPWR: u32 = 0b1 << 11;      // LCD power enable



// frame buffer memory address
const FRAMEBASE: u32 = 0x20000;

pub fn init() {
    unsafe {
        // Load timing registers for 800x600 resolution [pl110-trm 4.7.2].
        *SYS_OSC4 = 0x2cac;
        *LCDTIM0 = 0x1313A4C4;
        *LCDTIM1 = 0x0505F657;
        *LCDTIM2 = 0x071F1800;
        // Load address of frame buffer.
        *LCDUPBASE = FRAMEBASE;
        // Set control flags.
        *LCDCTRL = LCDEN | LCDBPP_24 | LCDTFT | LCDPWR;
    }
}
