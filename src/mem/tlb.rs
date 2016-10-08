pub fn flush_all() {
    unsafe {
        asm!("TLBI VMALLE1");
    }
}
