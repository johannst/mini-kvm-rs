// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021, Johannes Stoelp <dev@memzero.de>

use kvm_rs::kvm::Kvm;
use kvm_rs::vcpu::KvmExit;
use kvm_rs::{PhysAddr, UserMem};

fn main() -> std::io::Result<()> {
    // Create VM & VCPU.
    let vm = Kvm::new()?.create_vm()?;
    let mut vcpu = vm.create_vpcu(0)?;

    // Map memory for guest VM and initialize with guest image.
    let mem = UserMem::with_init(0x1000, include_bytes!("../guest/guest16"))?;
    unsafe {
        vm.set_user_memory_region(PhysAddr(0x0), &mem)?;
    }

    // Initialize VPCU registers.
    let mut regs = vcpu.get_regs()?;
    regs.rip = 0;
    regs.rflags = 0x2;
    vcpu.set_regs(regs)?;

    // Initialize VPCU special registers.
    let mut sregs = vcpu.get_sregs()?;
    sregs.cs.base = 0;
    sregs.cs.selector = 0;
    vcpu.set_sregs(sregs)?;

    // Run VCPU until `hlt` instruction.
    while let Ok(exit) = vcpu.run() {
        match exit {
            KvmExit::Halt => break,
            KvmExit::IoIn(port, data) => {
                println!("IO_IN: port={} len={}", port, data.len());
                // Provide some input data.
                data.fill(0xaa);
            }
            KvmExit::IoOut(_port, data) => {
                let s = std::str::from_utf8(data).unwrap();
                print!("{}", s);
            }
            KvmExit::MmioRead(addr, data) => {
                println!("MMIO_READ: addr={:#x} len={}", addr, data.len());
                // Provide some read data.
                data.fill(0xbb);
            }
            KvmExit::MmioWrite(addr, data) => {
                println!(
                    "MMIO_WRITE: addr={:#x} len={} data={:#x?}",
                    addr,
                    data.len(),
                    data
                );
            }
            KvmExit::Debug(_pc) => {}
        };
    }

    Ok(())
}
