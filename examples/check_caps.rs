use kvm_rs::cap::CapBool::*;
use kvm_rs::cap::CapInt::*;
use kvm_rs::kvm::Kvm;

fn main() -> std::io::Result<()> {
    let kvm = Kvm::new()?;

    println!("KVM_CAP_CHECK_EXTENSION_VM    = {}", kvm.check_extenstion(CheckExtensionVm));
    println!("KVM_CAP_NR_VCPUS              = {}", kvm.check_extenstion_int(NrVcpus));
    println!("KVM_CAP_MAX_VCPUS             = {}", kvm.check_extenstion_int(MaxVcpus));

    Ok(())
}
