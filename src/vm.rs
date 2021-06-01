//! VM system ioctls.

use std::fs;
use std::io;
use std::os::unix::io::FromRawFd;

use crate::vcpu::Vcpu;
use crate::{ioctl, kvm_sys, KvmRun, PhysAddr, UserMem};

/// Wrapper for VM ioctls.
///
/// Representation of the file descriptor obtained by the [`KVM_CREATE_VM`][kvm-create-vm] ioctl.
/// This wrapper provides access to the `VM ioctls` as described in [KVM API][kvm].
///
/// [kvm]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#general-description
/// [kvm-create-vm]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-create-vm
pub struct Vm {
    vm: fs::File,
    vcpu_mmap_size: usize,
}

impl Vm {
    pub(crate) fn new(vm: fs::File, vcpu_mmap_size: usize) -> Vm {
        Vm { vm, vcpu_mmap_size }
    }

    /// Map memory from userspace into the VM as `guest physical` memory starting at address
    /// `phys_addr`.
    /// The underlying operation is the [`KVM_SET_USER_MEMORY_REGION`][kmv-set-user-memory-region]
    /// ioctl.
    ///
    /// # Safety
    ///
    /// The `mem: &UserMem` argument passed to this function must at least live as long the `Vcpu`
    /// instance.
    ///
    /// [kvm-set-user-memory-region]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-set-user-memory-region
    pub unsafe fn set_user_memory_region(
        &self,
        phys_addr: PhysAddr,
        mem: &UserMem,
    ) -> io::Result<()> {
        // Create guest physical memory mapping for `slot : 0` at guest `phys_addr`.
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

    /// Create a new virtual cpu with the [`KVM_CREATE_VCPU`][kvm-create-vcpu] ioctl.
    /// Returns a wrapper [`vcpu::Vcpu`][crate::vcpu::Vcpu] representing the VCPU.
    ///
    /// [kvm-create-vcpu]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-create-vcpu
    pub fn create_vpcu(&self, id: u64) -> io::Result<Vcpu> {
        let vcpu = ioctl(&self.vm, kvm_sys::KVM_CREATE_VCPU, id)
            .map(|fd| unsafe { fs::File::from_raw_fd(fd) })?;

        let kvm_run = KvmRun::new(&vcpu, self.vcpu_mmap_size)?;

        Ok(Vcpu::new(vcpu, kvm_run))
    }
}
