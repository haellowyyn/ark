//! Driver for the ARM PrimeCell Color LCD Controller (PL110).
//! ref: "ARM PrimeCell Color LCD Controller (PL110) Technical Reference Manual" [pl110-trm]
//!
//! The PL110 implemented in the versatilepb differs slightly from the specification
//! in [pl110-trm]. See [versatilepb-ug 4.7.1].

use board::SYS_OSC4;
use board::CLCDC_BASE;

// CLCDC register offsets [pl110-trm 3.1]
// horizontal axis panel control register
const LCDTIM0: *mut u32 = (CLCDC_BASE + 0x000) as *mut u32;
// vertical axis panal control register
const LCDTIM1: *mut u32 = (CLCDC_BASE + 0x004) as *mut u32;
// clock and signal polarity control register
const LCDTIM2: *mut u32 = (CLCDC_BASE + 0x008) as *mut u32;
// upper panel frame base address register
const LCDUPBASE: *mut u32 = (CLCDC_BASE + 0x010) as *mut u32;
// control register
const LCDCTRL: *mut u32 = (CLCDC_BASE + 0x018) as *mut u32;

// LCDCTRL flags [pl110-trm 3.2.7]
const LCDEN: u32 = 0b1 << 0;        // LCD controller enable
const LCDBPP_24: u32 = 0b101 << 1;  // LCD bits per pixel: 24
const LCDTFT: u32 = 0b1 << 5;       // LCD if TFT
const LCDPWR: u32 = 0b1 << 11;      // LCD power enable


pub unsafe fn init(width: usize, height: usize, framebase: u32) {
    // Load timing registers depending on the resolution [pl110-trm 4.7.2].
    let (osc4, tim0, tim1, tim2) = match (width, height) {
        (240, 320) => (0x2C77, 0xC7A7BF38, 0x595B613F, 0x04eF1800),
        (320, 240) => (0x2C77, 0x9F7FBF4C, 0x818360eF, 0x053F1800),
        (176, 220) => (0x2C77, 0xe7C7BF28, 0x8B8D60DB, 0x04AF1800),
        (640, 480) => (0x2C77, 0x3F1F3F9C, 0x090B61DF, 0x067F1800),
        (800, 600) => (0x2cac, 0x1313A4C4, 0x0505F657, 0x071F1800),
        _ => panic!("unsupported screen resolution"),
    };
    *SYS_OSC4 = osc4;
    *LCDTIM0 = tim0;
    *LCDTIM1 = tim1;
    *LCDTIM2 = tim2;

    // Load address of frame buffer.
    *LCDUPBASE = framebase;
    // Set control flags.
    *LCDCTRL = LCDEN | LCDBPP_24 | LCDTFT | LCDPWR;
}
