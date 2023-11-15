var searchIndex = JSON.parse('{\
"kvm_rs":{"doc":"","t":"DDLLLLLLALLLLLAALLLLLLLLAALAEENNNLLLLLLLLLLLLLLLLDLLLLLLLLLLLMMMMLLLLLLLLLLMMMMMMMMLLLLLMMMMMMMLLLLLLLLLLLLLLMMMMMMLLLLLDDDDDMMMMMMMMMMMMMMMMMMMMMMMMMMMLLLLMLLLLLLLLLLMLLLLLNNNNENNDLLLLLLLLLLLLLLLLLLLLLLDLLLLLLLLLRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRFFFFFFFFFFFFFF","n":["PhysAddr","UserMem","as_mut","as_ref","borrow","borrow","borrow_mut","borrow_mut","cap","drop","from","from","into","into","kvm","kvm_sys","load","new","try_from","try_from","try_into","try_into","type_id","type_id","vcpu","vm","with_init","x86_64","CapBool","CapInt","CheckExtensionVm","MaxVcpus","NrVcpus","borrow","borrow","borrow_mut","borrow_mut","from","from","into","into","into","into","try_from","try_from","try_into","try_into","type_id","type_id","Kvm","borrow","borrow_mut","check_extenstion","check_extenstion_int","create_vm","from","into","new","try_from","try_into","type_id","apic_base","avl","base","base","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cr0","cr2","cr3","cr4","cr8","cs","db","db","default","default","default","default","default","dpl","dr6","dr7","ds","efer","es","flags","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","fs","g","gdt","gs","idt","interrupt_bitmap","into","into","into","into","into","kvm_debugregs","kvm_dtable","kvm_regs","kvm_segment","kvm_sregs","l","ldt","limit","limit","present","r10","r11","r12","r13","r14","r15","r8","r9","rax","rbp","rbx","rcx","rdi","rdx","reserved","rflags","rip","rsi","rsp","s","selector","ss","to_string","to_string","to_string","to_string","tr","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_","type_id","type_id","type_id","type_id","type_id","Debug","Halt","IoIn","IoOut","KvmExit","MmioRead","MmioWrite","Vcpu","borrow","borrow","borrow_mut","borrow_mut","from","from","get_debugregs","get_regs","get_sregs","into","into","run","set_debugregs","set_regs","set_single_step","set_sregs","try_from","try_from","try_into","try_into","type_id","type_id","Vm","borrow","borrow_mut","create_vpcu","from","into","set_user_memory_region","try_from","try_into","type_id","CR0_AM","CR0_CD","CR0_EM","CR0_ET","CR0_MP","CR0_NE","CR0_NW","CR0_PE","CR0_PG","CR0_TS","CR0_WP","CR3_PAGE_BASE_MASK","CR3_PCD","CR3_PWT","CR4_LA57","CR4_PAE","EFER_LMA","EFER_LME","MSR_EFER","PAGE_ENTRY_PRESENT","PAGE_ENTRY_RW","RFLAGS_AC","RFLAGS_AF","RFLAGS_CF","RFLAGS_DF","RFLAGS_IF","RFLAGS_IOPL","RFLAGS_OF","RFLAGS_PF","RFLAGS_SF","RFLAGS_TF","RFLAGS_ZF","SEG_SELECTOR_INDEX","SEG_SELECTOR_RPL","SEG_SELECTOR_TI","rflags_ac","rflags_af","rflags_cf","rflags_df","rflags_if","rflags_iopl","rflags_of","rflags_pf","rflags_sf","rflags_tf","rflags_zf","seg_selector_index","seg_selector_rpl","seg_selector_ti"],"q":[[0,"kvm_rs"],[28,"kvm_rs::cap"],[49,"kvm_rs::kvm"],[61,"kvm_rs::kvm_sys"],[173,"kvm_rs::vcpu"],[203,"kvm_rs::vm"],[213,"kvm_rs::x86_64"]],"d":["Strong type representing physical addresses.","Wrapper to safely allocate memory for guest VMs.","","","","","","","Definitions of KVM capabilities.","Free underlying memory.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","KVM system ioctls.","Definitions of the system header <code>&lt;linux/kvm.h&gt;</code>.","Load the bytes stored in <code>data</code> into memory at physical …","Allocate a zero-initialized memory region of <code>len</code> bytes.","","","","","","","VCPU system ioctls.","VM system ioctls.","Allocate a zero-initialized memory region of <code>len</code> bytes and …","<code>x86_64</code> flags and bitfields.","Definition of capabilities that return a bool value …","Definition of capabilities that return an integer value …","Check if capabilities can be queried on VM fds (…","Get the possible max VPCUs (<code>KVM_CAP_MAX_VCPUS</code>).","Get the recommended max VPCUs (<code>KVM_CAP_NR_VCPUS</code>).","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","Calls <code>U::from(self)</code>.","","","","","","","Wrapper for <code>/dev/kvm</code> ioctls.","","","Check availability of an extension with the …","Check availability of an extension with the …","Create a new virtual machine with the <code>KVM_CREATE_VM</code> ioctl. …","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Open the <code>/dev/kvm</code> device.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Exit reasons for the <code>Vcpu::run</code> function.","","","Wrapper for VCPU ioctls.","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Get the debug registers with the <code>KVM_GET_DEBUGREGS</code> ioctl …","Get the general purpose registers with the <code>KVM_GET_REGS</code> …","Get the special registers with the <code>KVM_GET_SREGS</code> ioctl in …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Run the guest VCPU with the <code>KVM_RUN</code> ioctl until it exits …","Set the debug registers with the <code>KVM_SET_DEBUGREGS</code> ioctl …","Set the general purpose registers with the <code>KVM_SET_REGS</code> …","Enable or disable guest single steppig (debug) with the …","Set the special registers with the <code>KVM_SET_SREGS</code> ioctl in …","","","","","","","Wrapper for VM ioctls.","","","Create a new virtual cpu with the <code>KVM_CREATE_VCPU</code> ioctl. …","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Map memory from userspace into the VM as <code>guest physical</code> …","","","","Alignment Mask.","Cachine disable.","Emulation.","Extension Type.","Monitor Coprocessor.","Numeric Error.","Not Write-Torugh.","Protection Enable.","Paging.","Task Switched.","Write Protect.","Mask for physical base address of paging structure.","Page-level Cache Disable.","Page-level Write-Through.","57-bit Linear Addresses.","Physical Address Extenstion.","Long Mode Active (readonly).","Long Mode Enable.","Extended Feature Enable Register MSR number.","Page entry present.","Page region read/write.","Alignment check.","Adjust flag.","Carry flag.","Direction flag.","Sign flag.","I/O privilege level.","Overflow flag.","Parity flag.","Sign flag.","Trap flag.","Zero flag.","Table index.","Requested privilege level.","Table indicator.","","","","","","","","","","","","","",""],"i":[0,0,1,1,4,1,4,1,0,1,4,1,4,1,0,0,1,1,4,1,4,1,4,1,0,0,1,0,0,0,9,11,11,9,11,9,11,9,11,9,9,11,11,9,11,9,11,9,11,0,12,12,12,12,12,12,12,12,12,12,12,19,17,17,18,16,17,18,19,20,16,17,18,19,20,19,19,19,19,19,19,17,20,16,17,18,19,20,17,20,20,19,19,19,20,16,16,17,17,18,18,19,19,20,16,17,18,19,20,19,17,19,19,19,19,16,17,18,19,20,0,0,0,0,0,17,19,17,18,17,16,16,16,16,16,16,16,16,16,16,16,16,16,16,20,16,16,16,16,17,17,19,16,17,18,19,19,16,17,18,19,20,16,17,18,19,20,17,16,17,18,19,20,25,25,25,25,0,25,25,0,25,24,25,24,25,24,24,24,24,25,24,24,24,24,24,24,25,24,25,24,25,24,0,15,15,15,15,15,15,15,15,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[0,0,[1,[[3,[2]]]],[1,[[3,[2]]]],[[]],[[]],[[]],[[]],0,[1],[[]],[[]],[[]],[[]],0,0,[[1,4,[3,[2]]]],[5,[[6,[1]]]],[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],0,0,[[5,[3,[2]]],[[6,[1]]]],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[9,10],[11,10],[[]],[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],0,[[]],[[]],[[12,9],13],[[12,11],14],[12,[[6,[15]]]],[[]],[[]],[[],[[6,[12]]]],[[],7],[[],7],[[],8],0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,[[],16],[[],17],[[],18],[[],19],[[],20],0,0,0,0,0,0,0,[[16,21],22],[[16,21],22],[[17,21],22],[[17,21],22],[[18,21],22],[[18,21],22],[[19,21],22],[[19,21],22],[[20,21],22],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],23],[[],23],[[],23],[[],23],0,[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],0,[[],8],[[],8],[[],8],[[],8],[[],8],0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[24,[[6,[20]]]],[24,[[6,[16]]]],[24,[[6,[19]]]],[[]],[[]],[24,[[6,[25]]]],[[24,20],6],[[24,16],6],[[24,13],6],[[24,19],6],[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],0,[[]],[[]],[[15,10],[[6,[24]]]],[[]],[[]],[[15,4,1],6],[[],7],[[],7],[[],8],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[10,10],[10,10],[10,10],[10,10],[10,10],[10,10],[10,10],[10,10],[10,10],[10,10],[10,10],[26,26],[26,26],[26,26]],"c":[],"p":[[3,"UserMem"],[15,"u8"],[15,"slice"],[3,"PhysAddr"],[15,"usize"],[6,"Result"],[4,"Result"],[3,"TypeId"],[4,"CapBool"],[15,"u64"],[4,"CapInt"],[3,"Kvm"],[15,"bool"],[15,"i32"],[3,"Vm"],[3,"kvm_regs"],[3,"kvm_segment"],[3,"kvm_dtable"],[3,"kvm_sregs"],[3,"kvm_debugregs"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"Vcpu"],[4,"KvmExit"],[15,"u16"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
