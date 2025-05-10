// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021, Johannes Stoelp <dev@memzero.de>

//! `x86_64` flags and bitfields.

pub use x86_64::*;

#[rustfmt::skip]
mod x86_64 {
    /* Rflags Register */

    /// Carry flag.
    pub const RFLAGS_CF: u64 = 1 << 0;
    /// Parity flag.
    pub const RFLAGS_PF: u64 = 1 << 2;
    /// Adjust flag.
    pub const RFLAGS_AF: u64 = 1 << 4;
    /// Zero flag.
    pub const RFLAGS_ZF: u64 = 1 << 6;
    /// Sign flag.
    pub const RFLAGS_SF: u64 = 1 << 7;
    /// Trap flag.
    pub const RFLAGS_TF: u64 = 1 << 8;
    /// Sign flag.
    pub const RFLAGS_IF: u64 = 1 << 9;
    /// Direction flag.
    pub const RFLAGS_DF: u64 = 1 << 10;
    /// Overflow flag.
    pub const RFLAGS_OF: u64 = 1 << 11;
    /// I/O privilege level.
    pub const RFLAGS_IOPL: u64 = 0b11 << 12;
    /// Alignment check.
    pub const RFLAGS_AC: u64 = 1 << 18;

    pub const fn rflags_cf(r: u64) -> u64   { (r & RFLAGS_CF)   >> 0 }
    pub const fn rflags_pf(r: u64) -> u64   { (r & RFLAGS_PF)   >> 2 }
    pub const fn rflags_af(r: u64) -> u64   { (r & RFLAGS_AF)   >> 4 }
    pub const fn rflags_zf(r: u64) -> u64   { (r & RFLAGS_ZF)   >> 6 }
    pub const fn rflags_sf(r: u64) -> u64   { (r & RFLAGS_SF)   >> 7 }
    pub const fn rflags_tf(r: u64) -> u64   { (r & RFLAGS_TF)   >> 8 }
    pub const fn rflags_if(r: u64) -> u64   { (r & RFLAGS_IF)   >> 9 }
    pub const fn rflags_df(r: u64) -> u64   { (r & RFLAGS_DF)   >> 10 }
    pub const fn rflags_of(r: u64) -> u64   { (r & RFLAGS_OF)   >> 11 }
    pub const fn rflags_iopl(r: u64) -> u64 { (r & RFLAGS_IOPL) >> 12 }
    pub const fn rflags_ac(r: u64) -> u64   { (r & RFLAGS_AC)   >> 18 }

    /* Segment Selector */

    /// Requested privilege level.
    ///
    /// Privilege level of the segment selector, where `0` is the most privileged mode and `3` the
    /// least.
    pub const SEG_SELECTOR_RPL: u16 = 0b11 << 0;
    /// Table indicator.
    ///
    /// | TI | Table |
    /// |----|-------|
    /// | 0  | GDT   |
    /// | 1  | LDT   |
    pub const SEG_SELECTOR_TI: u16 = 1 << 2;
    /// Table index.
    ///
    /// Index into the `GDT` or `LDT` table to select the segment descriptor. `GDT.base + 8 * index`
    /// gives the address of the segment descriptor (times `8` because every segment descriptor is `8
    /// byte`).
    pub const SEG_SELECTOR_INDEX: u16 = 0x1fff << 3;

    pub const fn seg_selector_rpl(s: u16) -> u16   { (s & SEG_SELECTOR_RPL)   >> 0 }
    pub const fn seg_selector_ti(s: u16) -> u16    { (s & SEG_SELECTOR_TI)    >> 2 }
    pub const fn seg_selector_index(s: u16) -> u16 { (s & SEG_SELECTOR_INDEX) >> 3 }

    /* Control Register CR0 (operation mode & state of the processor) */

    /// Protection Enable.
    ///
    /// Enables `protected mode` when set and `real-address mode` when cleared. This enables
    /// `segment-level protection` not paging.
    pub const CR0_PE: u64 = 1 << 0;
    /// Monitor Coprocessor.
    pub const CR0_MP: u64 = 1 << 1;
    /// Emulation.
    ///
    /// When set indicates the process does not have a FPU. FPU instructions will generate an exception
    /// that software can emulate the instruction.
    pub const CR0_EM: u64 = 1 << 2;
    /// Task Switched.
    pub const CR0_TS: u64 = 1 << 3;
    /// Extension Type.
    pub const CR0_ET: u64 = 1 << 4;
    /// Numeric Error.
    pub const CR0_NE: u64 = 1 << 5;
    /// Write Protect.
    ///
    /// When set supervisor-level procedures can't write to read-only pages.
    pub const CR0_WP: u64 = 1 << 16;
    /// Alignment Mask.
    ///
    /// Enables alignment check for `CPL=3`, check is only done if the [AC
    /// bit](crate::x86_64::RFLAGS_AC) of the `rflags` register ist set.
    pub const CR0_AM: u64 = 1 << 18;
    /// Not Write-Torugh.
    pub const CR0_NW: u64 = 1 << 29;
    /// Cachine disable.
    pub const CR0_CD: u64 = 1 << 30;
    /// Paging.
    ///
    /// Enables paging when set, requires [CR0_PE](crate::x86_64::CR0_PE) to be set as well.
    pub const CR0_PG: u64 = 1 << 31;

    /* Control Register CR3 (paging information)
     *
     * Holds the physical base address of the first paging structure. The 12 lower bytes of the base
     * address are assumed to be 0 and hence the first paging structure must be aligned to a 4K
     * boundary.
     */

    /// Mask for physical base address of paging structure.
    pub const CR3_PAGE_BASE_MASK: u64 = 0xffff_ffff_ffff_0000;

    /// Page-level Write-Through.
    pub const CR3_PWT: u64 = 1 << 3;
    /// Page-level Cache Disable.
    pub const CR3_PCD: u64 = 1 << 4;

    /* Control Register CR4 (flags for arch extenstions processor capabilities) */

    /// Physical Address Extenstion.
    ///
    /// When set enables paging to produce physicall addresses with more than 32 bits. Required before
    /// entering `long mode`.
    pub const CR4_PAE: u64 = 1 << 5;
    /// 57-bit Linear Addresses.
    ///
    /// When set in `long mode` enables `5-level` paging to translate `57-bit` linear addresses. When
    /// cleared use `4-level` paging to translate `48-bit` linear addresses.
    pub const CR4_LA57: u64 = 1 << 5;

    /* Extended Feature Enable Register (EFER) */

    /// Extended Feature Enable Register MSR number.
    ///
    /// MSR number used with the [`rdmsr`][msr] and [`wrmsr`][msr] instructions to read/write the
    /// `EFER` model specific register.
    ///
    /// [msr]: https://johannst.github.io/notes/arch/x86_64.html#model-specific-register-msr
    pub const MSR_EFER: u64 = 0xc000_0080;

    /// Long Mode Enable.
    ///
    /// When set enables long mode operations.
    pub const EFER_LME: u64 = 1 << 8;
    /// Long Mode Active (readonly).
    ///
    /// When set indicates long mode is active.
    pub const EFER_LMA: u64 = 1 << 10;

    /* Paging */

    /// Page entry present.
    pub const PAGE_ENTRY_PRESENT: u64 = 1 << 0;
    /// Page region read/write.
    ///
    /// If set, region reference by paging entry is writeable.
    pub const PAGE_ENTRY_RW: u64 = 1 << 1;
}
