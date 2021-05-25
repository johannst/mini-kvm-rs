use std::fs;
use std::io;
use std::os::unix::io::FromRawFd;

use crate::vcpu::Vcpu;
use crate::{ioctl, kvm_sys, KvmRun, PhysAddr, UserMem};

pub struct Vm {
    vm: fs::File,
    vcpu_mmap_size: usize,
}

impl Vm {
    pub(crate) fn new(vm: fs::File, vcpu_mmap_size: usize) -> Vm {
        Vm { vm, vcpu_mmap_size }
    }

    pub unsafe fn set_user_memory_region(
        &self,
        phys_addr: PhysAddr,
        mem: &UserMem,
    ) -> io::Result<()> {
        // Create guest physical memory mapping for `slot : 0` at guest `phys addr : 0`.
        let mut kvm_mem = kvm_sys::kvm_userspace_memory_region::default();
        kvm_mem.userspace_addr = mem.ptr as u64;
        kvm_mem.memory_size = mem.len as u64;
        kvm_mem.guest_phys_addr = phys_addr.0;

        ioctl(
            &self.vm,
            kvm_sys::KVM_SET_USER_MEMORY_REGION,
            &kvm_mem as *const _ as u64,
        )
        .map(|_| ())
    }

    pub fn create_vpcu(&self, id: u64) -> io::Result<Vcpu> {
        let vcpu = ioctl(&self.vm, kvm_sys::KVM_CREATE_VCPU, id)
            .map(|fd| unsafe { fs::File::from_raw_fd(fd) })?;

        let kvm_run = KvmRun::new(&vcpu, self.vcpu_mmap_size)?;

        Ok(Vcpu::new(vcpu, kvm_run))
    }
}
