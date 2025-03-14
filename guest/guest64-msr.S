.intel_syntax noprefix

// Define a string literal.
.macro defstr name str
\name:
    .ascii "\str"
\name\()_len:
    .8byte . - \name
.endm

// Print a string literal.
.macro pstr name
    mov dx, 0x42                   // Magic IO port for strings.
    lea rsi, [rip + \name]         // Address of string.
    mov rcx, [rip + \name\()_len]  // Len of string.
    rep outsb                      // Write ds:rsi to output port rdx.
.endm

.section .boot, "ax", @progbits
    // -- SWAPGS test -----------------------------------------------------------

    // Setup IA32_GS_BASE.
    mov rcx, 0xc0000101
    mov rax, 0xbbbbbbbb
    call wrmsr

    // Setup IA32_KERNEL_GS_BASE.
    mov rcx, 0xc0000102
    mov rax, 0xaaaaaaaa
    call wrmsr

    // Print IA32_GS_BASE.
    pstr gsbase
    mov rcx, 0xc0000101
    call rdmsr
    call pu64

    // Print IA32_KERNEL_GS_BASE.
    pstr kgsbase
    mov rcx, 0xc0000102
    call rdmsr
    call pu64

    pstr swapgs
    // Swap IA32_GS_BASE and IA32_KERNEL_GS_BASE.
    swapgs

    // Print IA32_GS_BASE.
    pstr gsbase
    mov rcx, 0xc0000101
    call rdmsr
    call pu64

    // Print IA32_KERNEL_GS_BASE.
    pstr kgsbase
    mov rcx, 0xc0000102
    call rdmsr
    call pu64

    // Trigger `KVM_EXIT_HLT`.
    hlt

// in : ecx address of msr register
// out: rax value of msr register
rdmsr:
    rdmsr
    shl rdx, 32
    or rax, rdx
    ret

// in: ecx address of msr register
// in: rax value to write into msr register
wrmsr:
    mov rdx, rax
    shr rdx, 32
    wrmsr
    ret

// in: rax holds 64bit word to be printed
pu64:
    xor dx, dx     // Port to print values.
    out dx, eax    // Print lower-half (u32) of rax.
    //shr rax, 32
    //out dx, eax    // Print upper-half (u32) of rax.
    ret

.section .rodata, "a", @progbits
defstr fsbase,  "fsbase: "
defstr gsbase,  "gsbase: "
defstr kgsbase, "kernel_gsbase: "
defstr swapgs,  "-- swapgs --\n"
