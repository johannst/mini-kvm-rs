//! Definitions of KVM capabilities.

use crate::kvm_sys;
use std::convert::Into;

/// Definition of capabilities that return a bool value indicating whether the capability is
/// supported or not.
#[repr(u64)]
pub enum CapBool {
    /// Check if capabilities can be queried on VM fds (`KVM_CAP_CHECK_EXTENSION_VM`).
    CheckExtensionVm = kvm_sys::KVM_CAP_CHECK_EXTENSION_VM,
}

impl Into<u64> for CapBool {
    fn into(self) -> u64 {
        self as u64
    }
}

/// Definition of capabilities that return an integer value indicating the amount of the queried
/// capability.
#[repr(u64)]
pub enum CapInt {
    /// Get the recommended max VPCUs (`KVM_CAP_NR_VCPUS`).
    NrVcpus = kvm_sys::KVM_CAP_NR_VCPUS,
    /// Get the possible max VPCUs (`KVM_CAP_MAX_VCPUS`).
    MaxVcpus = kvm_sys::KVM_CAP_MAX_VCPUS,
}

impl Into<u64> for CapInt {
    fn into(self) -> u64 {
        self as u64
    }
}
