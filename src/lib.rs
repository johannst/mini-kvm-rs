use std::convert::{AsMut, AsRef};
use std::io;
use std::ops;
use std::os::unix::io::AsRawFd;

pub mod kvm;
pub mod kvm_sys;
pub mod vcpu;
pub mod vm;

/// Strong type representing physical addresses.
pub struct PhysAddr(pub u64);

/// Helper to turn libc return values into an [io::Result](std::io::Result). Returns
/// [`Error::last_os_error`](std::io::Error::last_os_error) if `ret < 0`.
fn libcret(ret: libc::c_int) -> io::Result<libc::c_int> {
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}

/// Wrapper of `libc::ioctl` for KVM ioctls with one argument and returning an
/// [`io::Result`](std::io::Result).
fn ioctl<F: AsRawFd>(fd: &F, cmd: u64, arg: u64) -> io::Result<libc::c_int> {
    libcret(unsafe { libc::ioctl(fd.as_raw_fd(), cmd, arg) })
}

/// Wrapper to safely allocate memory for guest VMs.
///
/// The underlying memory is freed automatically once the `UserMem` instance is dropped.
///
/// Memory can be mapped into a guest VM with
/// [`Vm::set_user_memory_region`](crate::vm::Vm::set_user_memory_region).
pub struct UserMem {
    ptr: *mut u8,
    len: usize,
}

impl UserMem {
    /// Allocate a zero-initialized memory region of `len` bytes.
    pub fn new(len: usize) -> io::Result<UserMem> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            Err(io::Error::last_os_error())
        } else {
            Ok(UserMem {
                ptr: ptr.cast(),
                len,
            })
        }
    }

    /// Allocate a zero-initialized memory region of `len` bytes and initialize the first bytes
    /// with `init_from`.
    ///
    /// # Panics
    ///
    /// Panics if `init_from` is larger than the memory size `len`.
    pub fn with_init(len: usize, init_from: &[u8]) -> io::Result<UserMem> {
        assert!(len >= init_from.len());

        let mut m = UserMem::new(len)?;
        m.load(PhysAddr(0), init_from);
        Ok(m)
    }

    /// Load the bytes stored in `data` into memory at physical address `addr`.
    ///
    /// # Panics
    ///
    /// Panics if `addr + data.len` is larger than the memory size `len`.
    pub fn load(&mut self, addr: PhysAddr, data: &[u8]) {
        assert!(self.len >= addr.0 as usize + data.len());

        let addr = addr.0 as usize;
        self.as_mut()[addr..addr + data.len()].copy_from_slice(data);
    }
}

impl ops::Drop for UserMem {
    /// Free underlying memory.
    fn drop(&mut self) {
        unsafe { libc::munmap(self.ptr.cast(), self.len) };
    }
}

impl AsRef<[u8]> for UserMem {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl AsMut<[u8]> for UserMem {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

/// Internal wrapper to automatically `mmap` and `munmap` the the [`struct kvm_run`][kvm_run]
/// for a given VPCU.
///
/// [kvm_run]: https://www.kernel.org/doc/html/latest/virt/kvm/api.html#the-kvm-run-structure
struct KvmRun {
    ptr: *mut kvm_sys::kvm_run,
    len: usize,
}

impl KvmRun {
    /// Mmap the `struct kvm_run` for a given `VCPU` referenced by the argument file descriptor
    /// `vcpu`.
    fn new<F: AsRawFd>(vcpu: &F, len: usize) -> io::Result<KvmRun> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                libc::PROT_READ,
                libc::MAP_SHARED,
                vcpu.as_raw_fd(),
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            Err(io::Error::last_os_error())
        } else {
            Ok(KvmRun {
                ptr: ptr.cast(),
                len,
            })
        }
    }
}

impl ops::Drop for KvmRun {
    /// Munmap the mmaped `struct kvm_run`.
    fn drop(&mut self) {
        unsafe { libc::munmap(self.ptr.cast(), self.len) };
    }
}

impl AsRef<kvm_sys::kvm_run> for KvmRun {
    fn as_ref(&self) -> &kvm_sys::kvm_run {
        unsafe { &(*self.ptr) }
    }
}
