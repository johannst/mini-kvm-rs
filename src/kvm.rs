//! KVM system ioctls.

use std::fs;
use std::io;
use std::os::unix::io::FromRawFd;

use crate::{libcret, ioctl, kvm_sys};
use crate::vm::Vm;

/// Wrapper for `/dev/kvm` ioctls.
///
/// Representation of the file descriptor obtained by opening `/dev/kvm`.
/// This wrapper provides access to the `system ioctls` as described in [KVM API][kvm].
///
/// [kvm]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#general-description
pub struct Kvm {
    kvm: fs::File,
}

impl Kvm {
    /// Open the `/dev/kvm` device.
    pub fn new() -> io::Result<Kvm> {
        let kvm = libcret(unsafe {
            libc::open("/dev/kvm\0".as_ptr().cast(), libc::O_RDWR | libc::O_CLOEXEC)
        })
        .map(|fd| unsafe { fs::File::from_raw_fd(fd) })?;

        assert_eq!(
            kvm_sys::KVM_API_VERSION,
            ioctl(&kvm, kvm_sys::KVM_GET_API_VERSION, 0)?
        );

        Ok(Kvm { kvm })
    }

    fn get_vpcu_mmap_size(&self) -> io::Result<usize> {
        ioctl(&self.kvm, kvm_sys::KVM_GET_VCPU_MMAP_SIZE, 0).map(|size| size as usize)
    }

    /// Create a new virtual machine with the [`KVM_CREATE_VM`][kvm-create-vm] ioctl.
    /// Returns a wrapper [`vm::Vm`][crate::vm::Vm] representing the VM.
    ///
    /// [kvm-create-vm]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-create-vm
    pub fn create_vm(&self) -> io::Result<Vm> {
        let vm = ioctl(&self.kvm, kvm_sys::KVM_CREATE_VM, 0 /* machine id */)
            .map(|fd| unsafe { fs::File::from_raw_fd(fd) })?;

        let vcpu_mmap_size = self.get_vpcu_mmap_size()?;

        Ok(Vm::new(vm, vcpu_mmap_size))
    }
}
