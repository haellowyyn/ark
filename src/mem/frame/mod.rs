use spin::Mutex;
use super::PAddr;

mod allocator;


// We only use 4 KB frames.
pub const FRAME_SIZE: usize = 4 * 1024;

static ALLOCATOR: Mutex<allocator::Allocator> = Mutex::new(allocator::Allocator::new());


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
pub struct Frame(pub usize);

impl Frame {
    pub fn from_pa(pa: PAddr) -> Self {
        Frame(pa / FRAME_SIZE)
    }

    pub fn range_incl(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            curr: start,
            end: Frame(end.0 + 1),
        }
    }

    pub fn pa(self) -> PAddr {
        self.0 * FRAME_SIZE
    }
}


pub struct FrameIter {
    curr: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.curr < self.end {
            let tmp = self.curr;
            self.curr.0 += 1;
            Some(tmp)
        } else {
            None
        }
    }
}


pub fn alloc() -> Option<Frame> {
    let mut frame_allocator = ALLOCATOR.lock();
    frame_allocator.alloc()
}

pub fn free(frame: Frame) {
    let mut frame_allocator = ALLOCATOR.lock();
    frame_allocator.free(frame);
}
