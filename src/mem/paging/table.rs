use core::ops;

use mem::frame::Frame;
use mem::PAddr;


const NUM_ENTRIES: usize = 512;


pub struct Table([Entry; NUM_ENTRIES]);

impl Table {
    pub fn clear(&mut self) {
        for entry in self.0.iter_mut() {
            entry.unset();
        }
    }
}

impl ops::Index<usize> for Table {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.0[index]
    }
}


#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Entry(u64);

impl Entry {
    pub fn set(&mut self, frame: Frame, flags: u64) {
        let pa = frame.pa();
        assert!(pa & !0xfffffffff000 == 0);
        self.0 = pa as u64 | flags;
    }

    pub fn unset(&mut self) {
        self.0 = 0;
    }

    pub fn frame(self) -> Frame {
        let pa = (self.0 & 0xfffffffff000) as PAddr;
        Frame::from_pa(pa)
    }

    pub fn is_present(self) -> bool {
        self.0 & 0b1 == 0b1
    }

    pub fn refs_huge_page(self) -> bool {
        self.0 & 0b11 == 0b10
    }
}
