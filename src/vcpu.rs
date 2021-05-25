use std::fs;
use std::io;

use crate::{ioctl, kvm_sys, KvmRun};

pub enum KvmExit<'cpu> {
    Halt,
    IoOut(u16, &'cpu [u8]),
    MmioWrite(u64, &'cpu [u8]),
}

pub struct Vcpu {
    vcpu: fs::File,
    kvm_run: KvmRun,
}

impl Vcpu {
    pub(crate) fn new(vcpu: fs::File, kvm_run: KvmRun) -> Vcpu {
        Vcpu { vcpu, kvm_run }
    }

    pub fn get_regs(&self) -> io::Result<kvm_sys::kvm_regs> {
        let mut regs = kvm_sys::kvm_regs::default();
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_GET_REGS,
            &mut regs as *mut _ as u64,
        )?;
        Ok(regs)
    }

    pub fn set_regs(&self, regs: kvm_sys::kvm_regs) -> io::Result<()> {
        ioctl(&self.vcpu, kvm_sys::KVM_SET_REGS, &regs as *const _ as u64).map(|_| ())
    }

    pub fn get_sregs(&self) -> io::Result<kvm_sys::kvm_sregs> {
        let mut sregs = kvm_sys::kvm_sregs::default();
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_GET_SREGS,
            &mut sregs as *mut _ as u64,
        )?;
        Ok(sregs)
    }

    pub fn set_sregs(&self, sregs: kvm_sys::kvm_sregs) -> io::Result<()> {
        ioctl(
            &self.vcpu,
            kvm_sys::KVM_SET_SREGS,
            &sregs as *const _ as u64,
        )
        .map(|_| ())
    }

    pub fn run(&self) -> io::Result<KvmExit> {
        ioctl(&self.vcpu, kvm_sys::KVM_RUN, 0)?;

        let kvm_run = self.kvm_run.as_ref();

        match kvm_run.exit_reason as u64 {
            kvm_sys::KVM_EXIT_HLT => Ok(KvmExit::Halt),
            kvm_sys::KVM_EXIT_IO => {
                // Safe to use union `io` field, as Kernel instructed us to.
                let io = unsafe { kvm_run.inner.io };

                let kvm_run_ptr = kvm_run as *const kvm_sys::kvm_run as *const u8;

                // Create IO buffer located at `kvm_run + io.offset`.
                let data = unsafe {
                    std::slice::from_raw_parts(
                        kvm_run_ptr.offset(io.data_offset as isize),
                        io.count /* num blocks */ as usize * io.size /* bytes per block */ as usize,
                    )
                };

                match io.direction as u64 {
                    kvm_sys::KVM_EXIT_IO_IN => todo!("KVM_EXIT_IO_IN not implemented!"),
                    kvm_sys::KVM_EXIT_IO_OUT => Ok(KvmExit::IoOut(io.port, data)),
                    _ => unreachable!(),
                }
            }
            kvm_sys::KVM_EXIT_MMIO => {
                // Safe to use union `mmio` filed, as Kernel instructed us to.
                let mmio = unsafe { &kvm_run.inner.mmio };
                let len = mmio.len as usize;

                // Only support write at the moment.
                assert_ne!(0, mmio.is_write);

                Ok(KvmExit::MmioWrite(mmio.phys_addr, &mmio.data[..len]))
            }
            r @ _ => {
                todo!("KVM_EXIT_... (exit_reason={}) not implemented!", r)
            }
        }
    }
}
