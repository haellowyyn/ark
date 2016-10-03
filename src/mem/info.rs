use mem::PAddr;


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


macro_rules! sym_getter {
    ( $getter:ident, $sym:ident ) => {
        pub fn $getter() -> PAddr {
            unsafe {
                &$sym as *const _ as usize - &_kspace_start as *const _ as usize
            }
        }
    };
}

sym_getter!(krnl_start, _krnl_start);
sym_getter!(krnl_end, _krnl_end);

sym_getter!(text_start, _text_start);
sym_getter!(text_end, _text_end);

sym_getter!(rodata_start, _rodata_start);
sym_getter!(rodata_end, _rodata_end);

sym_getter!(data_start, _data_start);
sym_getter!(data_end, _data_end);

sym_getter!(bss_start, _bss_start);
sym_getter!(bss_end, _bss_end);

sym_getter!(stack_bottom, _stack_bottom);
sym_getter!(stack_top, _stack_top);
