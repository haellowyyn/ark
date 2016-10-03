use mem::info::{krnl_start, krnl_end};

use super::{Frame, FRAME_SIZE};


// Number of frames available in 256 MB of memory.
const NUM_FRAMES: usize = 256 * 1024 * 1024 / FRAME_SIZE;

// Number of 64-bit words needed for the allocator bitmap.
const NUM_BITMAP_WORDS: usize = NUM_FRAMES / 64;


pub struct Allocator {
    used: Bitmap, // tracks the used page frames
}

impl Allocator {
    pub const fn new() -> Self {
        Allocator { used: Bitmap([0; NUM_BITMAP_WORDS]) }
    }

    pub fn init(&mut self) {
        // Mark the frames used by the kernel as unavailable.
        let first_kframe = Frame::from_pa(krnl_start());
        let last_kframe = Frame::from_pa(krnl_end());

        for frame in Frame::range_incl(first_kframe, last_kframe) {
            self.used.set(frame.0);
        }
    }

    pub fn alloc(&mut self) -> Option<Frame> {
        self.used.first_free().and_then(|n| {
            self.used.set(n);
            Some(Frame(n))
        })
    }

    pub fn free(&mut self, frame: Frame) {
        self.used.clear(frame.0)
    }
}


struct Bitmap([u64; NUM_BITMAP_WORDS]);

impl Bitmap {
    fn set(&mut self, bit_nr: usize) {
        assert!(bit_nr < NUM_FRAMES,
                "bit_nr is {} but must not be larger than {}",
                bit_nr,
                NUM_FRAMES);

        let mut word = &mut self.0[bit_nr / 64];
        *word |= 1 << (bit_nr % 64);
    }

    fn clear(&mut self, bit_nr: usize) {
        assert!(bit_nr < NUM_FRAMES,
                "bit_nr is {} but must not be larger than {}",
                bit_nr,
                NUM_FRAMES);

        let mut word = &mut self.0[bit_nr / 64];
        *word &= !(1 << (bit_nr % 64));
    }

    fn first_free(&self) -> Option<usize> {
        for (i, &word) in self.0.iter().enumerate() {
            if word == 0xffffffffffffffff {
                continue;
            }
            for word_bit in 0..64 {
                if word & (1 << word_bit) == 0 {
                    return Some(i * 64 + word_bit);
                }
            }
        }
        None
    }
}
