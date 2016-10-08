use mem::{PAddr, VAddr};


// Linker symbols that have no associated value.
// Only use their addresses.
#[allow(improper_ctypes)]
extern "C" {
    static _kspace_start: ();
    static _krnl_start: ();
    static _krnl_end: ();
    static _text_start: ();
    static _text_end: ();
    static _rodata_start: ();
    static _rodata_end: ();
    static _data_start: ();
    static _data_end: ();
    static _bss_start: ();
    static _bss_end: ();
    static _stack_bottom: ();
    static _stack_top: ();
}


macro_rules! va_sym {
    ( $getter:ident, $sym:ident ) => {
        pub fn $getter() -> VAddr {
            unsafe { &$sym as *const _ as usize }
        }
    };
}


macro_rules! pa_sym {
    ( $getter:ident, $sym:ident ) => {
        pub fn $getter() -> PAddr {
            unsafe {
                &$sym as *const _ as usize - &_kspace_start as *const _ as usize
            }
        }
    };
}

va_sym!(kspace_start, _kspace_start);

pa_sym!(krnl_start, _krnl_start);
pa_sym!(krnl_end, _krnl_end);

pa_sym!(text_start, _text_start);
pa_sym!(text_end, _text_end);

pa_sym!(rodata_start, _rodata_start);
pa_sym!(rodata_end, _rodata_end);

pa_sym!(data_start, _data_start);
pa_sym!(data_end, _data_end);

pa_sym!(bss_start, _bss_start);
pa_sym!(bss_end, _bss_end);

// pa_sym!(stack_bottom, _stack_bottom);
// pa_sym!(stack_top, _stack_top);
