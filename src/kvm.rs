// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021, Johannes Stoelp <dev@memzero.de>

//! KVM system ioctls.

use std::fs;
use std::io;
use std::os::unix::io::FromRawFd;

use crate::cap::{CapBool, CapInt};
use crate::vm::Vm;
use crate::{ioctl, kvm_sys, libcret};

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

    /// Check availability of an extension with the [`KVM_CHECK_EXTENSION`][kvm-check-extension]
    /// ioctl.
    ///
    /// [kvm-check-extension]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-check-extension
    pub fn check_extenstion(&self, cap: CapBool) -> bool {
        let ret = ioctl(&self.kvm, kvm_sys::KVM_CHECK_EXTENSION, cap.into());

        matches!(ret, Ok(ret) if ret > 0)
    }

    /// Check availability of an extension with the [`KVM_CHECK_EXTENSION`][kvm-check-extension]
    /// ioctl.
    ///
    /// [kvm-check-extension]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-check-extension
    pub fn check_extenstion_int(&self, cap: CapInt) -> i32 {
        let ret = ioctl(&self.kvm, kvm_sys::KVM_CHECK_EXTENSION, cap.into());

        ret.unwrap_or(0)
    }
}
