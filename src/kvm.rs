use std::fs;
use std::io;
use std::os::unix::io::FromRawFd;

use crate::{libcret, ioctl, kvm_sys};
use crate::vm::Vm;

pub struct Kvm {
    kvm: fs::File,
}

impl Kvm {
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

    pub fn create_vm(&self) -> io::Result<Vm> {
        let vm = ioctl(&self.kvm, kvm_sys::KVM_CREATE_VM, 0 /* machine id */)
            .map(|fd| unsafe { fs::File::from_raw_fd(fd) })?;

        let vcpu_mmap_size = self.get_vpcu_mmap_size()?;

        Ok(Vm::new(vm, vcpu_mmap_size))
    }
}
