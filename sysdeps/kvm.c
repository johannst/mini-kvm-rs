#include <linux/kvm.h>

#include <stdio.h>
#include <stdalign.h> // alignof operator
#include <stddef.h>   // offsetof operator

int main() {
    /* Global constants */

    printf("pub(crate) const KVM_API_VERSION : i32 = 0x%x;\n", KVM_API_VERSION);

    /* ioctl's for /dev/kvm */

    // param: none
    // ret  : constant KVM_API_VERSION (=12)
    printf("pub(crate) const KVM_GET_API_VERSION : u64 = 0x%x;\n", KVM_GET_API_VERSION);
    // param: machine type identifier (most cases just 0)
    // ret  : VM fd
    printf("pub(crate) const KVM_CREATE_VM : u64 = 0x%x;\n", KVM_CREATE_VM);
    // param: none
    // ret  : size of vcpu mmap region in bytes
    printf("pub(crate) const KVM_GET_VCPU_MMAP_SIZE : u64 = 0x%x;\n", KVM_GET_VCPU_MMAP_SIZE);

    /* ioctl's for VM fd */

    // param: vpcu id
    // ret  : VCPU fd
    printf("pub(crate) const KVM_CREATE_VCPU : u64 = 0x%x;\n", KVM_CREATE_VCPU);
    // param: struct kvm_userspace_memory_region
    // ret  : 0 success, -1 error
    printf("pub(crate) const KVM_SET_USER_MEMORY_REGION : u64 = 0x%lx;\n", KVM_SET_USER_MEMORY_REGION);

    /* ioctl's for VCPU fd */

    // param: none
    // ret  : 0 success, -1 error
    printf("pub(crate) const KVM_RUN : u64 = 0x%x;\n", KVM_RUN);
    // param: struct kvm_regs
    // ret  : 0 success, -1 error
    printf("pub(crate) const KVM_GET_REGS : u64 = 0x%lx;\n", KVM_GET_REGS);
    // param: struct kvm_regs
    // ret  : 0 success, -1 error
    printf("pub(crate) const KVM_SET_REGS : u64 = 0x%lx;\n", KVM_SET_REGS);
    // param: struct kvm_sregs
    // ret  : 0 success, -1 error
    printf("pub(crate) const KVM_GET_SREGS : u64 = 0x%lx;\n", KVM_GET_SREGS);
    // param: struct kvm_sregs
    // ret  : 0 success, -1 error
    printf("pub(crate) const KVM_SET_SREGS : u64 = 0x%lx;\n", KVM_SET_SREGS);

    /* struct kvm_run constants */

    printf("pub(crate) const KVM_EXIT_HLT : u64 = 0x%x;\n", KVM_EXIT_HLT);
    printf("pub(crate) const KVM_EXIT_IO : u64 = 0x%x;\n", KVM_EXIT_IO);
    printf("pub(crate) const KVM_EXIT_IO_IN : u64 = 0x%x;\n", KVM_EXIT_IO_IN);
    printf("pub(crate) const KVM_EXIT_IO_OUT : u64 = 0x%x;\n", KVM_EXIT_IO_OUT);
    printf("pub(crate) const KVM_EXIT_MMIO : u64 = 0x%x;\n", KVM_EXIT_MMIO);

    /* Testing constants */

    printf("#[cfg(test)] const TEST_KVM_REGS_SIZE : usize = %ld;\n", sizeof(struct kvm_regs));
    printf("#[cfg(test)] const TEST_KVM_REGS_ALIGN : usize = %ld;\n", alignof(struct kvm_regs));
    printf("#[cfg(test)] const TEST_KVM_SREGS_SIZE : usize = %ld;\n", sizeof(struct kvm_sregs));
    printf("#[cfg(test)] const TEST_KVM_SREGS_ALIGN : usize = %ld;\n", alignof(struct kvm_sregs));
    printf("#[cfg(test)] const TEST_KVM_SREGS_INTERRTUP_BITMAP_SIZE : usize = %ld;\n", sizeof(((struct kvm_sregs*)0)->interrupt_bitmap));
    printf("#[cfg(test)] const TEST_KVM_SEGMENT_SIZE : usize = %ld;\n", sizeof(struct kvm_segment));
    printf("#[cfg(test)] const TEST_KVM_SEGMENT_ALIGN : usize = %ld;\n", alignof(struct kvm_segment));
    printf("#[cfg(test)] const TEST_KVM_DTABLE_SIZE : usize = %ld;\n", sizeof(struct kvm_dtable));
    printf("#[cfg(test)] const TEST_KVM_DTABLE_ALIGN : usize = %ld;\n", alignof(struct kvm_dtable));
    printf("#[cfg(test)] const TEST_KVM_USERSPACE_MEMORY_REGION_SIZE : usize = %ld;\n", sizeof(struct kvm_userspace_memory_region));
    printf("#[cfg(test)] const TEST_KVM_USERSPACE_MEMORY_REGION_ALIGN : usize = %ld;\n", alignof(struct kvm_userspace_memory_region));
    printf("#[cfg(test)] const TEST_KVM_RUN_SIZE : usize = %ld;\n", sizeof(struct kvm_run));
    printf("#[cfg(test)] const TEST_KVM_RUN_ALIGN : usize = %ld;\n", alignof(struct kvm_run));
    printf("#[cfg(test)] const TEST_KVM_RUN_IO_SIZE : usize = %ld;\n", sizeof(((struct kvm_run*)0)->io));
    printf("#[cfg(test)] const TEST_KVM_RUN_MMIO_SIZE : usize = %ld;\n", sizeof(((struct kvm_run*)0)->mmio));
    printf("#[cfg(test)] const TEST_KVM_RUN_UNION_S_SIZE : usize = %ld;\n", sizeof(((struct kvm_run*)0)->s));

    return 0;
}
