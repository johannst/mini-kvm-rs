OUTPUT_FORMAT(binary)

SECTIONS {
    .boot     : { *(.boot) }
    .text     : { *(.text) }
    .data     : { *(.data) }
    .rodata   : { *(.rodata) }
    /DISCARD/ : { *(.*) }
}
