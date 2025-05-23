// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021, Johannes Stoelp <dev@memzero.de>

//! Definitions of the system header [`<linux/kvm.h>`][kvm-h].
//!
//! [kvm-h]: https://elixir.bootlin.com/linux/latest/source/include/uapi/linux/kvm.h

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// Generated by `build.rs`.
include!(concat!(env!("OUT_DIR"), "/kvm_constants.rs"));

#[repr(C)]
#[derive(Default, Debug)]
pub struct kvm_regs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct kvm_segment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub type_: u8,
    pub present: u8,
    pub dpl: u8,
    pub db: u8,
    pub s: u8,
    pub l: u8,
    pub g: u8,
    pub avl: u8,
    unusable: u8,
    _padding: u8,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct kvm_dtable {
    pub base: u64,
    pub limit: u16,
    _padding: [u16; 3],
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct kvm_sregs {
    pub cs: kvm_segment,
    pub ds: kvm_segment,
    pub es: kvm_segment,
    pub fs: kvm_segment,
    pub gs: kvm_segment,
    pub ss: kvm_segment,
    pub tr: kvm_segment,
    pub ldt: kvm_segment,
    pub gdt: kvm_dtable,
    pub idt: kvm_dtable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
    pub apic_base: u64,
    pub interrupt_bitmap: [u64; 4],
}

#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct kvm_userspace_memory_region {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Default, Debug)]
pub struct kvm_debugregs {
    pub db: [u64; 4],
    pub dr6: u64,
    pub dr7: u64,
    pub flags: u64,
    pub reserved: [u64; 9],
}

#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct kvm_guest_debug {
    pub control: u32,
    pad: u32,
    pub arch: kvm_guest_debug_arch,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct kvm_guest_debug_arch {
    pub debugreg: [u64; 8],
}

#[repr(C)]
pub(crate) struct kvm_run {
    request_interrupt_window: u8,
    immediate_exit: u8,
    padding1: [u8; 6],
    pub exit_reason: u32,
    ready_for_interrupt_injection: u8,
    if_flag: u8,
    flags: u16,
    cr8: u64,
    apic_base: u64,
    pub inner: kvm_run_union,
    kvm_valid_regs: u64,
    kvm_dirty_regs: u64,
    s: kvm_run_union_s,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct kvm_run_io {
    pub direction: u8,
    pub size: u8,
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct kvm_run_mmio {
    pub phys_addr: u64,
    pub data: [u8; 8],
    pub len: u32,
    pub is_write: u8,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct kvm_run_debug {
    pub exception: u32,
    pad: u32,
    pub pc: u64,
    pub dr6: u64,
    pub dr7: u64,
}

// Only add the union fields used here.
#[repr(C)]
pub(crate) union kvm_run_union {
    pub io: kvm_run_io,
    pub mmio: kvm_run_mmio,
    pub debug: kvm_run_debug,
    padding: [u8; 256],
}

// Only add the union fields used here.
#[repr(C)]
union kvm_run_union_s {
    padding: [u8; 2048],
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn check_kvm_regs() {
        assert_eq!(mem::size_of::<kvm_regs>(), TEST_KVM_REGS_SIZE);
        assert_eq!(mem::align_of::<kvm_regs>(), TEST_KVM_REGS_ALIGN);
    }

    #[test]
    fn check_kvm_segment() {
        assert_eq!(mem::size_of::<kvm_segment>(), TEST_KVM_SEGMENT_SIZE);
        assert_eq!(mem::align_of::<kvm_segment>(), TEST_KVM_SEGMENT_ALIGN);
    }

    #[test]
    fn check_kvm_dtable() {
        assert_eq!(mem::size_of::<kvm_dtable>(), TEST_KVM_DTABLE_SIZE);
        assert_eq!(mem::align_of::<kvm_dtable>(), TEST_KVM_DTABLE_ALIGN);
    }

    #[test]
    fn check_kvm_sregs() {
        assert_eq!(mem::size_of::<kvm_sregs>(), TEST_KVM_SREGS_SIZE);
        assert_eq!(mem::align_of::<kvm_sregs>(), TEST_KVM_SREGS_ALIGN);
        assert_eq!(
            mem::size_of_val(&kvm_sregs::default().interrupt_bitmap),
            TEST_KVM_SREGS_INTERRTUP_BITMAP_SIZE
        );
    }

    #[test]
    fn check_kvm_userspace_memory_region() {
        assert_eq!(
            mem::size_of::<kvm_userspace_memory_region>(),
            TEST_KVM_USERSPACE_MEMORY_REGION_SIZE
        );
        assert_eq!(
            mem::align_of::<kvm_userspace_memory_region>(),
            TEST_KVM_USERSPACE_MEMORY_REGION_ALIGN
        );
    }

    #[test]
    fn check_kvm_run() {
        assert_eq!(mem::size_of::<kvm_run>(), TEST_KVM_RUN_SIZE);
        assert_eq!(mem::align_of::<kvm_run>(), TEST_KVM_RUN_ALIGN);
        assert_eq!(mem::size_of::<kvm_run_io>(), TEST_KVM_RUN_IO_SIZE);
        assert_eq!(mem::size_of::<kvm_run_mmio>(), TEST_KVM_RUN_MMIO_SIZE);
        assert_eq!(mem::size_of::<kvm_run_union_s>(), TEST_KVM_RUN_UNION_S_SIZE);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn check_kvm_run_x86() {
        assert_eq!(mem::size_of::<kvm_run_debug>(), TEST_KVM_RUN_DEBUG_SIZE);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn check_kvm_debugregs() {
        assert_eq!(mem::size_of::<kvm_debugregs>(), TEST_KVM_DEBUGREGS_SIZE);
        assert_eq!(mem::align_of::<kvm_debugregs>(), TEST_KVM_DEBUGREGS_ALIGN);
    }

    #[test]
    fn check_kvm_guest_dbg() {
        assert_eq!(mem::size_of::<kvm_guest_debug>(), TEST_KVM_GUEST_DEBUG_SIZE);
        assert_eq!(
            mem::align_of::<kvm_guest_debug>(),
            TEST_KVM_GUEST_DEBUG_ALIGN
        );
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn check_kvm_guest_dbg_arch() {
        assert_eq!(
            mem::size_of::<kvm_guest_debug_arch>(),
            TEST_KVM_GUEST_DEBUG_ARCH_SIZE
        );
        assert_eq!(
            mem::align_of::<kvm_guest_debug_arch>(),
            TEST_KVM_GUEST_DEBUG_ARCH_ALIGN
        );
    }
}
