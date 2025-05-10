// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021, Johannes Stoelp <dev@memzero.de>

//! VCPU system ioctls.

use std::fs;
use std::io;

use crate::{ioctl, kvm_sys, KvmRun};

/// Exit reasons for the [`Vcpu::run`][crate::vcpu::Vcpu::run] function.
///
/// Details for the different exit reasons can be found in the [`kvm_run`
/// structure][kvm-run-struct] description.
///
/// [kvm-run]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-run
/// [kvm-run-struct]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#the-kvm-run-structure
pub enum KvmExit<'cpu> {
    Halt,
    IoIn(u16, &'cpu mut [u8]),
    IoOut(u16, &'cpu [u8]),
    MmioRead(u64, &'cpu mut [u8]),
    MmioWrite(u64, &'cpu [u8]),
    Debug(u64),
}

/// Wrapper for VCPU ioctls.
///
/// Representation of the file descriptor obtained by the [`KVM_CREATE_VCPU`][kvm-create-vcpu] ioctl.
/// This wrapper provides access to the `VCPU ioctls` as described in [KVM API][kvm].
///
/// [kvm]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#general-description
/// [kvm-create-vcpu]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-create-vcpu
pub struct Vcpu {
    vcpu: fs::File,
    kvm_run: KvmRun,
}

impl Vcpu {
    pub(crate) fn new(vcpu: fs::File, kvm_run: KvmRun) -> Vcpu {
        Vcpu { vcpu, kvm_run }
    }

    /// Get the general purpose registers with the [`KVM_GET_REGS`][kvm-get-regs] ioctl in form of
    /// [`kvm_regs`](crate::kvm_sys::kvm_regs).
    ///
    /// [kvm-get-regs]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-get-regs
    pub fn get_regs(&self) -> io::Result<kvm_sys::kvm_regs> {
        let mut regs = kvm_sys::kvm_regs::default();
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_GET_REGS,
            &mut regs as *mut _ as u64,
        )?;
        Ok(regs)
    }

    /// Set the general purpose registers with the [`KVM_SET_REGS`][kvm-set-regs] ioctl in form of
    /// [`kvm_regs`](crate::kvm_sys::kvm_regs).
    ///
    /// [kvm-set-regs]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-set-regs
    pub fn set_regs(&self, regs: kvm_sys::kvm_regs) -> io::Result<()> {
        ioctl(&self.vcpu, kvm_sys::KVM_SET_REGS, &regs as *const _ as u64).map(|_| ())
    }

    /// Get the special registers with the [`KVM_GET_SREGS`][kvm-get-sregs] ioctl in form of
    /// [`kvm_sregs`](crate::kvm_sys::kvm_sregs).
    ///
    /// [kvm-get-sregs]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-get-sregs
    pub fn get_sregs(&self) -> io::Result<kvm_sys::kvm_sregs> {
        let mut sregs = kvm_sys::kvm_sregs::default();
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_GET_SREGS,
            &mut sregs as *mut _ as u64,
        )?;
        Ok(sregs)
    }

    /// Set the special registers with the [`KVM_SET_SREGS`][kvm-set-sregs] ioctl in form of
    /// [`kvm_sregs`](crate::kvm_sys::kvm_sregs).
    ///
    /// [kvm-set-sregs]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-set-sregs
    pub fn set_sregs(&self, sregs: kvm_sys::kvm_sregs) -> io::Result<()> {
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_SET_SREGS,
            &sregs as *const _ as u64,
        )
        .map(|_| ())
    }

    /// Get the debug registers with the [`KVM_GET_DEBUGREGS`][kvm-get-debugregs] ioctl in form of
    /// [`kvm_debugregs`](crate::kvm_sys::kvm_debugregs).
    ///
    /// [kvm-get-debugregs]:
    /// https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-get-debugregs
    #[cfg(target_arch = "x86_64")]
    pub fn get_debugregs(&self) -> io::Result<kvm_sys::kvm_debugregs> {
        let mut dregs = kvm_sys::kvm_debugregs::default();
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_GET_DEBUGREGS,
            &mut dregs as *mut _ as u64,
        )?;
        Ok(dregs)
    }

    /// Set the debug registers with the [`KVM_SET_DEBUGREGS`][kvm-set-debugregs] ioctl in form of
    /// [`kvm_debugregs`](crate::kvm_sys::kvm_debugregs).
    ///
    /// [kvm-set-debugregs]:
    /// https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-set-debugregs
    #[cfg(target_arch = "x86_64")]
    pub fn set_debugregs(&self, dregs: kvm_sys::kvm_debugregs) -> io::Result<()> {
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_SET_DEBUGREGS,
            &dregs as *const _ as u64,
        )
        .map(|_| ())
    }

    /// Enable or disable guest single steppig (debug) with the
    /// [`KVM_GUESTDBG_ENABLE`][kvm-guest-debug] ioctl.
    ///
    /// [kvm-guest-debug]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-set-guest-debug
    #[cfg(target_arch = "x86_64")]
    pub fn set_single_step(&self, enable: bool) -> io::Result<()> {
        let mut dbg = kvm_sys::kvm_guest_debug::default();

        if enable {
            // Enable guest debugging and single stepping.
            dbg.control = kvm_sys::KVM_GUESTDBG_ENABLE | kvm_sys::KVM_GUESTDBG_SINGLESTEP;
        }

        // Initialize debug registers based on current VCPUs debug register values.
        let dregs = self.get_debugregs()?;
        dbg.arch.debugreg[0..4].copy_from_slice(&dregs.db);
        // DR4-DR5 are reserved.
        dbg.arch.debugreg[6] = dregs.dr6;
        dbg.arch.debugreg[7] = dregs.dr7;

        ioctl(
            &self.vcpu,
            kvm_sys::KVM_SET_GUEST_DEBUG,
            &dbg as *const _ as u64,
        )
        .map(|_| ())
    }

    /// Run the guest VCPU with the [`KVM_RUN`][kvm-run] ioctl until it exits with one of the exit
    /// reasons described in [`KvmExit`](crate::vcpu::KvmExit).
    ///
    /// [kvm-run]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-run
    pub fn run(&mut self) -> io::Result<KvmExit<'_>> {
        ioctl(&self.vcpu, kvm_sys::KVM_RUN, 0)?;

        let kvm_run = self.kvm_run.as_mut();

        match kvm_run.exit_reason as u64 {
            kvm_sys::KVM_EXIT_HLT => Ok(KvmExit::Halt),
            kvm_sys::KVM_EXIT_IO => {
                // Safe to use union `io` field, as Kernel instructed us to.
                let io = unsafe { kvm_run.inner.io };

                let kvm_run_ptr = kvm_run as *mut kvm_sys::kvm_run as *mut u8;

                // Create IO buffer located at `kvm_run + io.offset`.
                let data = unsafe {
                    std::slice::from_raw_parts_mut(
                        kvm_run_ptr.offset(io.data_offset as isize),
                        io.count /* num blocks */ as usize * io.size /* bytes per block */ as usize,
                    )
                };

                match io.direction as u64 {
                    kvm_sys::KVM_EXIT_IO_IN => Ok(KvmExit::IoIn(io.port, data)),
                    kvm_sys::KVM_EXIT_IO_OUT => Ok(KvmExit::IoOut(io.port, data)),
                    _ => unreachable!(),
                }
            }
            kvm_sys::KVM_EXIT_MMIO => {
                // Safe to use union `mmio` filed, as Kernel instructed us to.
                let mmio = unsafe { &mut kvm_run.inner.mmio };
                let len = mmio.len as usize;

                match mmio.is_write {
                    0 => Ok(KvmExit::MmioRead(mmio.phys_addr, &mut mmio.data[..len])),
                    1 => Ok(KvmExit::MmioWrite(mmio.phys_addr, &mmio.data[..len])),
                    _ => unreachable!(),
                }
            }
            kvm_sys::KVM_EXIT_DEBUG => {
                // Safe to use union `debug` field, as Kernel instructed us to.
                let debug = unsafe { kvm_run.inner.debug };

                Ok(KvmExit::Debug(debug.pc))
            }
            r @ _ => {
                todo!("KVM_EXIT_... (exit_reason={}) not implemented!", r)
            }
        }
    }
}
