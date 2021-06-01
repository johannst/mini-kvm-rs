.code16
.intel_syntax noprefix

.section .boot, "ax", @progbits
    // Trigger `KVM_EXIT_IO:KVM_EXIT_IO_OUT` by writing string to output port.
    mov dx, 0x1000          // Output port.
    lea si, [msg]           // Address of string.
    mov cx, [msg_len]       // Len of string.
    rep outsb               // Write byte from ds:si to output port dx.

    // Trigger `KVM_EXIT_IO:KVM_EXIT_IO_IN` by reading byte to memory from input port.
    mov dx, 0x1000          // Input port.
    lea di, [in_dest]       // Destination address.
    insb                    // Read byte from input port dx to ds:di.

    // Trigger `KVM_EXIT_MMIO (w)` by writing to non mapped physical address.
    mov byte ptr ds:[0x2000], 0xaa

    // Trigger `KVM_EXIT_MMIO (r)` by reading from non mapped physical address.
    mov al, byte ptr ds:[0x2000]

    // Trigger `KVM_EXIT_HLT`.
    hlt

.section .rodata, "a", @progbits
msg:
    .asciz "Hello from Real Mode!\n"
msg_len:
    .2byte .-msg

in_dest:
    .byte 0x00
