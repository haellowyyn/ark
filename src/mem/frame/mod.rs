use spin::Mutex;
use super::PAddr;

mod allocator;


// We only use 4 KB frames.
pub const FRAME_SIZE: usize = 4 * 1024;

pub static ALLOCATOR: Mutex<allocator::Allocator> = Mutex::new(allocator::Allocator::new());


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
pub struct Frame(pub usize);

impl Frame {
    fn from_pa(pa: PAddr) -> Self {
        Frame(pa / FRAME_SIZE)
    }

    fn range_incl(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            curr: start,
            end: Frame(end.0 + 1),
        }
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
