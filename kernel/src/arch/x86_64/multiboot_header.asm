section .multiboot_header
header_start:
    ; Magic number (multiboot 2)
    dd 0xE85250D6
    ; Architecture 0 (protected mode i386)
    dd 0
    ; Header length
    dd header_end - header_start
    ; Checksum
    dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

    ; TODO insert optional multiboot stuff here

    ; End tag
    ; type
    dw 0
    ; flags
    dw 0
    ; size
    dd 8
header_end:
