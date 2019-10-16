SECTIONS {
    . = 0x00001000;
    .text : { *(.text) }
    . = 0x00002000;
    .data : { *(.data) }
}

