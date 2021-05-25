.code16
.intel_syntax noprefix

.section .boot, "ax", @progbits
    // Trigger `KVM_EXIT_IO:KVM_EXIT_IO_OUT` by writing string to output port.
    mov dx, 0x1000          // Output port.
    lea si, [msg]           // Address of string.
    mov cx, [msg_len]       // Len of string.
    rep outsb dx, ds:[si]   // Write out string bytes.

    // Trigger `KVM_EXIT_MMIO` by writing to non mapped physical address.
    mov byte ptr ds:[0x2000], 0xaa

    // Trigger `KVM_EXIT_HLT`.
    hlt

.section .rodata, "a", @progbits
msg:
    .asciz "Hello from Real Mode!\n"
msg_len:
    .byte .-msg
