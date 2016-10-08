//! The mem.init module exports only a single unsafe function `init_mm`.
//! `init_mm` must be called during kernel startup to initialize proper
//! memory management. When `init_mm` is called, the physical memory
//! must still be identity mapped into virtual memory.

use board::MMIO_REGIONS;
use super::info::*;
use super::{PAddr, frame, paging, tlb};
use super::frame::Frame;
use super::paging::Page;

pub unsafe fn init_mm() {
    {
        let ktbl_root = build_kernel_table();
        let utbl_root = build_user_table();
        switch_tables(ktbl_root, utbl_root);
        tlb::flush_all();
    }
}

/// Build the kernel translation table.
/// The kernel translation table directly maps the whole kernel.
fn build_kernel_table() -> Frame {
    let ktbl_root = must_alloc_frame();
    let mut l1_tbl = unsafe { table_from_frame(ktbl_root) };
    l1_tbl.clear();

    let kss = kspace_start();

    let (ts, te) = (text_start(), text_end());
    println!("  krnl.text:   {:#x}-{:#x}", ts + kss, te + kss);
    idmap_region(ts, te, l1_tbl);

    let (rds, rde) = (rodata_start(), rodata_end());
    println!("  krnl.rodata: {:#x}-{:#x}", rds + kss, rde + kss);
    idmap_region(rds, rde, l1_tbl);

    let (ds, de) = (data_start(), data_end());
    println!("  krnl.data:   {:#x}-{:#x}", ds + kss, de + kss);
    idmap_region(ds, de, l1_tbl);

    let (bs, be) = (bss_start(), bss_end());
    println!("  krnl.bss:    {:#x}-{:#x}", bs + kss, be + kss);
    idmap_region(bs, be, l1_tbl);

    // Map MMIO regions.
    for &(mrs, mre) in MMIO_REGIONS {
        println!("  MMIO region: {:#x}-{:#x}", mrs + kss, mre + kss);
        idmap_region(mrs, mre, l1_tbl);
    }

    ktbl_root
}

/// Build the kernel translation table.
/// The user translation table is empty for now.
fn build_user_table() -> Frame {
    let utbl_root = must_alloc_frame();
    let mut l1_tbl = unsafe { table_from_frame(utbl_root) };
    l1_tbl.clear();
    utbl_root
}

fn idmap_region(start_pa: PAddr, end_pa: PAddr, l1_tbl: &mut paging::Table) {
    let sf = Frame::from_pa(start_pa);
    let ef = Frame::from_pa(end_pa);
    for frame in Frame::range_incl(sf, ef) {
        identity_map(frame, l1_tbl);
    }
}

fn identity_map(frame: Frame, l1_tbl: &mut paging::Table) {
    // PA and VA are the same in an identity mapping.
    let page = Page::from_va(frame.pa());

    let mut l2_tbl = next_table(l1_tbl, page.l1_index());
    let mut l3_tbl = next_table(l2_tbl, page.l2_index());
    l3_tbl[page.l3_index()].set(frame, 0x403);
}

fn next_table(tbl: &mut paging::Table, index: usize) -> &mut paging::Table {
    if !tbl[index].is_present() {
        let next_root = must_alloc_frame();
        let mut next_tbl = unsafe { table_from_frame(next_root) };
        next_tbl.clear();
        tbl[index].set(next_root, 0x3);
        next_tbl
    } else {
        // We only support 4 KB pages.
        assert!(!tbl[index].refs_huge_page());
        let next_root = tbl[index].frame();
        unsafe { table_from_frame(next_root) }
    }
}

unsafe fn switch_tables(ktbl_root: Frame, utbl_root: Frame) {
    set_sysreg!("TTBR0_EL1", utbl_root.pa());
    set_sysreg!("TTBR1_EL1", ktbl_root.pa());
}

fn must_alloc_frame() -> Frame {
    frame::alloc().expect("allocating frame failed.")
}

unsafe fn table_from_frame<'a>(frame: Frame) -> &'a mut paging::Table {
    &mut *(frame.pa() as *mut paging::Table)
}
