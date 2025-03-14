.intel_syntax noprefix

.section .boot, "ax", @progbits
    // Trigger `KVM_EXIT_IO:KVM_EXIT_IO_OUT` by writing string to output port.
    mov rdx, 0x42               // Output port (magic for string fmt).
    lea rsi, [rip + msg]        // Address of string.
    mov rcx, [rip + msg_len]    // Len of string.
    rep outsb                   // Write ds:rsi to output port rdx.

    // Trigger `KVM_EXIT_IO:KVM_EXIT_IO_IN` by reading byte to memory from input port.
    mov dx, 0x1000          // Input port.
    lea di, [rip + in_dest] // Destination address.
    insb                    // Read byte from input port dx to ds:di.

    // Write to allocated virtual addresses.
    mov byte ptr ds:[0x2000], 0xaa
    mov byte ptr ds:[0x2001], 0xbb
    mov byte ptr ds:[0x2002], 0xcc
    mov byte ptr ds:[0x2003], 0xdd

    // Write to virtually mapped by not physically mapped address, this should
    // trigger a `KVM_EXIT_MMIO (w)`.
    mov byte ptr ds:[0x4000], 0x12
    mov byte ptr ds:[0x4001], 0x34
    mov byte ptr ds:[0x4002], 0x56
    mov byte ptr ds:[0x4003], 0x78

    // Trigger `KVM_EXIT_HLT`.
    hlt

.section .rodata, "a", @progbits
msg:
    .ascii "Hello from Long Mode!\n"
msg_len:
    .byte .-msg

in_dest:
    .byte 0x00
