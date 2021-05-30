.intel_syntax noprefix

.section .boot, "ax", @progbits
    // Trigger `KVM_EXIT_IO:KVM_EXIT_IO_OUT` by writing string to output port.
    mov rdx, 0x1000             // Output port.
    lea rsi, [rip + msg]        // Address of string.
    mov rcx, [rip + msg_len]    // Len of string.
    rep outsb                   // Write ds:rsi to output port rdx.

    // Trigger `KVM_EXIT_HLT`.
    hlt

.section .rodata, "a", @progbits
msg:
    .asciz "Hello from Long Mode!\n"
msg_len:
    .byte .-msg
