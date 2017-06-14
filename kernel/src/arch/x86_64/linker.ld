ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* Make sure the multiboot header is at the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}
