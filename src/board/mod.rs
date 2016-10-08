//! Code specific to the 'versatilepb' board.
//! ref: "RealView Platform Baseboard for ARM926EJ-S User Guide" [virtualpb-ug]

use mem::PAddr;


macro_rules! mmio_ptr {
    ( $pa:expr, $t:ty ) => {{
        use mem::info::kspace_start;
        ($pa + kspace_start()) as *mut $t
    }}
}


pub mod clcdc;
pub mod uart;


// MMIO regions that must be mapped into the kernel translation table.
// Instead of mapping the whole MMIO space of the board, we only map
// those regions that we actually access. [virtualpb-ug 4.1]
#[allow(unused_attributes)]
#[rustfmt_skip]
pub const MMIO_REGIONS: &'static [(PAddr, PAddr)] = &[
    (0x10000000, 0x10000FFF),  // System registers
    (0x10120000, 0x1012FFFF),  // Color LCD controller
    (0x101F1000, 0x101F1FFF),  // UART0 interface
];


// status and system control registers [virtualpb-ug 4.3]
const SYS_OSC4: PAddr = 0x1000001C;

// UART0 memory base address [virtualpb-ug 4.25]
const UART0_BASE: PAddr = 0x101f1000;

// CLCDC memory base address [virtualpb-ug 4.7]
const CLCDC_BASE: PAddr = 0x10120000;
