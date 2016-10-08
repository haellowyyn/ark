pub use self::table::Table;
use super::VAddr;
use super::frame::FRAME_SIZE;

mod table;


// Pages must fit into frames (4 KB).
pub const PAGE_SIZE: usize = FRAME_SIZE;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
pub struct Page(usize);

impl Page {
    pub fn from_va(va: VAddr) -> Self {
        Page(va / PAGE_SIZE)
    }

    pub fn l1_index(self) -> usize {
        (self.0 >> 18) & 0o777
    }

    pub fn l2_index(self) -> usize {
        (self.0 >> 9) & 0o777
    }

    pub fn l3_index(self) -> usize {
        (self.0 >> 0) & 0o777
    }
}
