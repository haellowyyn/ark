pub mod info;
mod frame;


type PAddr = usize;

/// Initialize memory management.
pub fn init_mm() {
    {
        let mut frame_allocator = frame::ALLOCATOR.lock();
        frame_allocator.init();
    }
}
