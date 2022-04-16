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
            r @ _ => {
                todo!("KVM_EXIT_... (exit_reason={}) not implemented!", r)
            }
        }
    }
}
