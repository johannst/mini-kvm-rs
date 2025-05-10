/* SPDX-License-Identifier: MIT
 *
 * Copyright (c) 2021, Johannes Stoelp <dev@memzero.de>
 */

OUTPUT_FORMAT(binary)

SECTIONS {
    .boot     : { *(.boot) }
    .text     : { *(.text) }
    .data     : { *(.data) }
    .rodata   : { *(.rodata) }
    /DISCARD/ : { *(.*) }
}
