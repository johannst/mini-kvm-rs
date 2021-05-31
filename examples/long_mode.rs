use kvm_rs::kvm::Kvm;
use kvm_rs::kvm_sys;
use kvm_rs::vcpu::KvmExit;
use kvm_rs::x86_64::*;
use kvm_rs::{PhysAddr, UserMem};

fn setup_long_mode_segments(sregs: &mut kvm_sys::kvm_sregs) {
    let code_seg = |seg: &mut kvm_sys::kvm_segment| {
        seg.base = 0x0;
        // Limit (unused in 64bit).
        seg.limit = 0x0;
        // Segment selector -> Index=1, Table=GDT, RPL=0.
        seg.selector = 0x8;
        // Code read + execute.
        seg.type_ = 10;
        // Segment present.
        seg.present = 1;
        // Descriptor privilege level.
        seg.dpl = 0;
        // Operand/Pointer size (unused in 64bit, but must be 0 in 64bit mode).
        seg.db = 0;
        // Code/data segment.
        seg.s = 1;
        // Native 64 bit code segment.
        seg.l = 1;
        // Granularity (unused in 64bit).
        seg.g = 0;
    };

    let data_seg = |seg: &mut kvm_sys::kvm_segment| {
        seg.base = 0x0;
        // Limit (unused in 64bit).
        seg.limit = 0x0;
        // Segment selector -> Index=2, Table=GDT, RPL=0.
        seg.selector = 0x10;
        // Data read + write.
        seg.type_ = 2;
        // Segment present.
        seg.present = 1;
        // Descriptor privilege level.
        seg.dpl = 0;
        // Operand/Pointer size (unused in 64bit, but must be 0 in 64bit mode).
        seg.db = 0;
        // Code/data segment.
        seg.s = 1;
        // Native 64 bit code segment.
        seg.l = 0;
        // Granularity (unused in 64bit).
        seg.g = 0;
    };

    code_seg(&mut sregs.cs);
    data_seg(&mut sregs.ds);
    data_seg(&mut sregs.ss);
    data_seg(&mut sregs.fs);
    data_seg(&mut sregs.gs);
    data_seg(&mut sregs.es);
}

fn setup_long_mode_4level_paging(mem: &mut UserMem) -> PhysAddr {
    assert_eq!(0x8000, mem.as_ref().len());

    // As a small exercise we create the following 4-level virtual address mapping using 4K pages:
    //     VirtAddr [0x0000:0x3fff] -> PhysAddr [0x4000:0x7fff]
    //
    // The required paging structures we'll place at physical address [0x0000:0x3ffff].
    //
    //        PhysAddr
    //        +-------+
    // 0x0000 | PML4  |
    // 0x1000 | PDP   |
    // 0x2000 | PD    |
    // 0x3000 | PT    |        VirtAddr
    // 0x4000 +-------+ <----- +-------+ 0x0000
    //        | Guest |        | Guest |
    //        | (16K) |        | (16K) |
    // 0x8000 +-------+ <----- +-------+ 0x4000
    //
    // PML4 : Page Map Level 4
    // PDP  : Page Directory Pointer
    // PD   : Page Directory
    // PT   : Page Table
    //
    // PML4, PDP, PD will contain a single entry at index 0.
    // PT will contain 4 page table entries (PTE) at index {0,1,2,3} -> 4 * 4K = 16K.

    let mut w = |addr: PhysAddr, val: u64| mem.load(addr, &val.to_le_bytes());

    // PML4E[0] refers to PDPE[0:4095].
    w(PhysAddr(0x0000), PAGE_ENTRY_PRESENT | PAGE_RENTRY_RW | 0x1000);
    // PDPE[0] refers to PDE[0:4095].
    w(PhysAddr(0x1000), PAGE_ENTRY_PRESENT | PAGE_RENTRY_RW | 0x2000);
    // PDE[0] refers to PTE[0:4095].
    w(PhysAddr(0x2000), PAGE_ENTRY_PRESENT | PAGE_RENTRY_RW | 0x3000);

    // PTE[0] maps Virt [0x0000:0x0fff] -> Phys [0x4000:0x4fff].
    w(PhysAddr(0x3000), PAGE_ENTRY_PRESENT | PAGE_RENTRY_RW | 0x4000);
    // PTE[1] maps Virt [0x1000:0x1fff] -> Phys [0x5000:0x5fff].
    w(PhysAddr(0x3008), PAGE_ENTRY_PRESENT | PAGE_RENTRY_RW | 0x5000);
    // PTE[2] maps Virt [0x2000:0x2fff] -> Phys [0x6000:0x6fff].
    w(PhysAddr(0x3010), PAGE_ENTRY_PRESENT | PAGE_RENTRY_RW | 0x6000);
    // PTE[3] maps Virt [0x3000:0x3fff] -> Phys [0x7000:0x7fff].
    w(PhysAddr(0x3018), PAGE_ENTRY_PRESENT | PAGE_RENTRY_RW | 0x7000);

    // Return address of PML4.
    PhysAddr(0x0000)
}

fn setup_long_mode(sregs: &mut kvm_sys::kvm_sregs, mem: &mut UserMem) {
    // Setup segment descriptors for long mode.
    setup_long_mode_segments(sregs);

    // Setup paging structures.
    let pml4_base = setup_long_mode_4level_paging(mem);
    // Setup physical address of first paging structure (PML4).
    sregs.cr3 = pml4_base.0;

    // Enable paging (PG) and protection (PE).
    sregs.cr0 = CR0_PG | CR0_PE;
    // Enable physical address extension (PAE).
    sregs.cr4 = CR4_PAE;
    // Set long mode active (LMA) and long mode enabled (LME).
    sregs.efer = EFER_LMA | EFER_LME;
}

fn main() -> std::io::Result<()> {
    // Create VM & VCPU.
    let vm = Kvm::new()?.create_vm()?;
    let vcpu = vm.create_vpcu(0)?;

    // Map memory for guest VM.
    let mut mem = UserMem::new(0x8000)?;
    unsafe {
        vm.set_user_memory_region(PhysAddr(0x0), &mem)?;
    }

    // Load guest image at physical address starting from 0x4000.
    mem.load(PhysAddr(0x4000), include_bytes!("../guest/guest64"));

    // Initialize VPCU registers.
    let mut regs = vcpu.get_regs()?;
    // Set `rip` to 0 as we want to start executing from virtual address 0.
    regs.rip = 0;
    regs.rflags = 0x2;
    vcpu.set_regs(regs)?;

    // Initialize VPCU special registers.
    let mut sregs = vcpu.get_sregs()?;
    // Setup long mode and paging to map:
    //     VirtAddr [0x0000:0x3fff] -> PhysAddr [0x4000:0x7fff]
    setup_long_mode(&mut sregs, &mut mem);
    vcpu.set_sregs(sregs)?;

    // Run VCPU until `hlt` instruction.
    while let Ok(exit) = vcpu.run() {
        match exit {
            KvmExit::Halt => break,
            KvmExit::IoOut(_port, data) => {
                let s = std::str::from_utf8(data).unwrap();
                print!("{}", s);
            }
            KvmExit::MmioWrite(addr, data) => {
                println!(
                    "MMIO_WRITE: addr={:#x} len={} data={:#x?}",
                    addr,
                    data.len(),
                    data
                );
            }
        };
    }

    Ok(())
}
