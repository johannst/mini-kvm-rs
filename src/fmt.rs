use std::fmt;

use crate::kvm_sys::{kvm_dtable, kvm_regs, kvm_segment, kvm_sregs};
use crate::x86_64::*;

impl fmt::Display for kvm_regs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rax: {:#018x} rbx: {:#018x} rcx: {:#018x} rdx: {:#018x}\n\
             rsi: {:#018x} rdi: {:#018x}\n\
             r8 : {:#018x} r9 : {:#018x} r10: {:#018x} r11: {:#018x}\n\
             r12: {:#018x} r13: {:#018x} r14: {:#018x} r15: {:#018x}\n\
             rsp: {:#018x} rbp: {:#018x}\n\
             rip: {:#018x} rfl: {:#018x} O({}) D({}) I({}) T({}) S({}) Z({}) P({}) C({})",
            self.rax,
            self.rbx,
            self.rcx,
            self.rdx,
            self.rsi,
            self.rdi,
            self.r8,
            self.r9,
            self.r10,
            self.r11,
            self.r12,
            self.r13,
            self.r14,
            self.r15,
            self.rsp,
            self.rbp,
            self.rip,
            self.rflags,
            rflags_of(self.rflags),
            rflags_df(self.rflags),
            rflags_if(self.rflags),
            rflags_tf(self.rflags),
            rflags_sf(self.rflags),
            rflags_zf(self.rflags),
            rflags_pf(self.rflags),
            rflags_cf(self.rflags),
        )
    }
}

impl fmt::Display for kvm_segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.present == 0 {
            write!(f, "{:#04x} P(0)", self.selector)
        } else {
            write!(
                f,
                "{:#06x} T({}) RPL({}) BASE({:#010x}) LIMIT({:#07x}) P(1) S({}) DPL({}) DB({}) L({}) TYPE({})",
                self.selector,
                if seg_selector_ti(self.selector) == 0 {
                    "GDT"
                } else {
                    "LDT"
                },
                seg_selector_rpl(self.selector),
                self.base,
                self.limit,
                self.s,
                self.dpl,
                self.db,
                self.l,
                match self.type_ {
                    0 => "D:R---",
                    1 => "D:R-A-",
                    2 => "D:RW--",
                    3 => "D:RWA-",
                    4 => "D:R--E",
                    5 => "D:R-AE",
                    6 => "D:RW-E",
                    7 => "D:RWAE",
                    8 => "C:X---",
                    9 => "C:X-A-",
                    10 => "C:XR--",
                    11 => "C:XRA-",
                    12 => "C:X--C",
                    13 => "C:X-AC",
                    14 => "C:XR-C",
                    15 => "C:XRAC",
                    _ => unreachable!(),
                }
            )
        }
    }
}

impl fmt::Display for kvm_dtable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BASE({:#018x}) LIMIT({:#07x})", self.base, self.limit)
    }
}

impl fmt::Display for kvm_sregs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cs  : {}\n\
             ds  : {}\n\
             es  : {}\n\
             fs  : {}\n\
             gs  : {}\n\
             ss  : {}\n\
             tr  : {}\n\
             ldt : {}\n\
             gdt : {}\n\
             idt : {}\n\
             cr0 : {:#018x} cr2: {:#018x} cr3: {:#018x} cr4: {:#018x}\n\
             efer: {:#018x}",
            self.cs,
            self.ds,
            self.es,
            self.fs,
            self.gs,
            self.ss,
            self.tr,
            self.ldt,
            self.gdt,
            self.idt,
            self.cr0,
            self.cr2,
            self.cr3,
            self.cr4,
            self.efer,
        )
    }
}
