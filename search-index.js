var searchIndex = JSON.parse('{\
"kvm_rs":{"doc":"","t":[12,3,3,11,11,11,11,11,11,0,11,11,11,11,11,0,0,11,11,11,11,11,11,11,11,0,0,11,0,4,4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,11,11,11,11,3,3,3,3,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,11,11,11,11,12,11,11,11,11,11,11,11,11,12,11,11,11,11,13,13,13,4,13,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,11,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,5,5,5,5,5,5,5,5,5,5,5,5,5],"n":["0","PhysAddr","UserMem","as_mut","as_ref","borrow","borrow","borrow_mut","borrow_mut","cap","drop","from","from","into","into","kvm","kvm_sys","load","new","try_from","try_from","try_into","try_into","type_id","type_id","vcpu","vm","with_init","x86_64","CapBool","CapInt","CheckExtensionVm","MaxVcpus","NrVcpus","borrow","borrow","borrow_mut","borrow_mut","from","from","into","into","into","into","try_from","try_from","try_into","try_into","type_id","type_id","Kvm","borrow","borrow_mut","check_extenstion","check_extenstion_int","create_vm","from","into","new","try_from","try_into","type_id","apic_base","avl","base","base","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cr0","cr2","cr3","cr4","cr8","cs","db","default","default","default","default","dpl","ds","efer","es","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","fs","g","gdt","gs","idt","interrupt_bitmap","into","into","into","into","kvm_dtable","kvm_regs","kvm_segment","kvm_sregs","l","ldt","limit","limit","present","r10","r11","r12","r13","r14","r15","r8","r9","rax","rbp","rbx","rcx","rdi","rdx","rflags","rip","rsi","rsp","s","selector","ss","to_string","to_string","to_string","to_string","tr","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_","type_id","type_id","type_id","type_id","Halt","IoIn","IoOut","KvmExit","MmioRead","MmioWrite","Vcpu","borrow","borrow","borrow_mut","borrow_mut","from","from","get_regs","get_sregs","into","into","run","set_regs","set_sregs","try_from","try_from","try_into","try_into","type_id","type_id","Vm","borrow","borrow_mut","create_vpcu","from","into","set_user_memory_region","try_from","try_into","type_id","CR0_AM","CR0_CD","CR0_EM","CR0_ET","CR0_MP","CR0_NE","CR0_NW","CR0_PE","CR0_PG","CR0_TS","CR0_WP","CR3_PAGE_BASE_MASK","CR3_PCD","CR3_PWT","CR4_LA57","CR4_PAE","EFER_LMA","EFER_LME","MSR_EFER","PAGE_ENTRY_PRESENT","PAGE_RENTRY_RW","RFLAGS_AC","RFLAGS_AF","RFLAGS_CF","RFLAGS_DF","RFLAGS_IF","RFLAGS_IOPL","RFLAGS_OF","RFLAGS_PF","RFLAGS_SF","RFLAGS_ZF","SEG_SELECTOR_INDEX","SEG_SELECTOR_RPL","SEG_SELECTOR_TI","rflags_ac","rflags_af","rflags_cf","rflags_df","rflags_if","rflags_iopl","rflags_of","rflags_pf","rflags_sf","rflags_zf","seg_selector_index","seg_selector_rpl","seg_selector_ti"],"q":["kvm_rs","","","","","","","","","","","","","","","","","","","","","","","","","","","","","kvm_rs::cap","","","","","","","","","","","","","","","","","","","","","kvm_rs::kvm","","","","","","","","","","","","kvm_rs::kvm_sys","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","kvm_rs::vcpu","","","","","","","","","","","","","","","","","","","","","","","","","","kvm_rs::vm","","","","","","","","","","kvm_rs::x86_64","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","Strong type representing physical addresses.","Wrapper to safely allocate memory for guest VMs.","","","","","","","Definitions of KVM capabilities.","Free underlying memory.","","","","","KVM system ioctls.","Definitions of the system header <code><linux/kvm.h></code>.","Load the bytes stored in <code>data</code> into memory at physical …","Allocate a zero-initialized memory region of <code>len</code> bytes.","","","","","","","VCPU system ioctls.","VM system ioctls.","Allocate a zero-initialized memory region of <code>len</code> bytes …","<code>x86_64</code> flags and bitfields.","Definition of capabilities that return a bool value …","Definition of capabilities that return an integer value …","Check if capabilities can be queried on VM fds (…","Get the possible max VPCUs (<code>KVM_CAP_MAX_VCPUS</code>).","Get the recommended max VPCUs (<code>KVM_CAP_NR_VCPUS</code>).","","","","","","","","","","","","","","","","","Wrapper for <code>/dev/kvm</code> ioctls.","","","Check availability of an extension with the …","Check availability of an extension with the …","Create a new virtual machine with the <code>KVM_CREATE_VM</code> ioctl.…","","","Open the <code>/dev/kvm</code> device.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Exit reasons for the [<code>Vcpu::kvm_run</code>][…","","","Wrapper for VCPU ioctls.","","","","","","","Get the general purpose registers with the <code>KVM_GET_REGS</code> …","Get the special registers with the <code>KVM_GET_SREGS</code> ioctl in …","","","Run the guest VCPU with the <code>KVM_RUN</code> ioctl until it exits …","Set the general purpose registers with the <code>KVM_SET_REGS</code> …","Set the special registers with the <code>KVM_SET_SREGS</code> ioctl in …","","","","","","","Wrapper for VM ioctls.","","","Create a new virtual cpu with the <code>KVM_CREATE_VCPU</code> ioctl. …","","","Map memory from userspace into the VM as <code>guest physical</code> …","","","","Alignment Mask.","Cachine disable.","Emulation.","Extension Type.","Monitor Coprocessor.","Numeric Error.","Not Write-Torugh.","Protection Enable.","Paging.","Task Switched.","Write Protect.","Mask for physical base address of paging structure.","Page-level Cache Disable.","Page-level Write-Through.","57-bit Linear Addresses.","Physical Address Extenstion.","Long Mode Active (readonly).","Long Mode Enable.","Extended Feature Enable Register MSR number.","Page entry present.","Page region read/write.","Alignment check.","Adjust flag.","Carry flag.","Direction flag.","Sign flag.","I/O privilege level.","Overflow flag.","Parity flag.","Sign flag.","Zero flag.","Table index.","Requested privilege level.","Table indicator.","","","","","","","","","","","","",""],"i":[1,0,0,2,2,1,2,1,2,0,2,1,2,1,2,0,0,2,2,1,2,1,2,1,2,0,0,2,0,0,0,3,4,4,3,4,3,4,3,4,3,3,4,4,3,4,3,4,3,4,0,5,5,5,5,5,5,5,5,5,5,5,6,7,7,8,9,7,8,6,9,7,8,6,6,6,6,6,6,6,7,9,7,8,6,7,6,6,6,9,9,7,7,8,8,6,6,9,7,8,6,6,7,6,6,6,6,9,7,8,6,0,0,0,0,7,6,7,8,7,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,7,7,6,9,7,8,6,6,9,7,8,6,9,7,8,6,7,9,7,8,6,10,10,10,0,10,10,0,10,11,10,11,10,11,11,11,10,11,11,11,11,10,11,10,11,10,11,0,12,12,12,12,12,12,12,12,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],[[]],null,null,[[["physaddr",3]]],[[["usize",15]],[["result",6],["usermem",3]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,[[["usize",15]],[["result",6],["usermem",3]]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["u64",15]],[[]],[[]],[[],["u64",15]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],[[["capbool",4]],["bool",15]],[[["capint",4]],["i32",15]],[[],[["result",6],["vm",3]]],[[]],[[]],[[],[["result",6],["kvm",3]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,null,null,null,[[],["kvm_regs",3]],[[],["kvm_segment",3]],[[],["kvm_dtable",3]],[[],["kvm_sregs",3]],null,null,null,null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],null,[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],[["result",6],["kvm_regs",3]]],[[],[["result",6],["kvm_sregs",3]]],[[]],[[]],[[],[["result",6],["kvmexit",4]]],[[["kvm_regs",3]],["result",6]],[[["kvm_sregs",3]],["result",6]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],[[["u64",15]],[["vcpu",3],["result",6]]],[[]],[[]],[[["physaddr",3],["usermem",3]],["result",6]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u64",15]],["u64",15]],[[["u16",15]],["u16",15]],[[["u16",15]],["u16",15]],[[["u16",15]],["u16",15]]],"p":[[3,"PhysAddr"],[3,"UserMem"],[4,"CapBool"],[4,"CapInt"],[3,"Kvm"],[3,"kvm_sregs"],[3,"kvm_segment"],[3,"kvm_dtable"],[3,"kvm_regs"],[4,"KvmExit"],[3,"Vcpu"],[3,"Vm"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};