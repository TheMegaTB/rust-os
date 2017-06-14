global long_mode_start

section .text
bits 64
long_mode_start:
    ; Load 0 into all data segment registers since some instructions expect it
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Call the rust main function
    extern rust_main
    call rust_main

    ; Print "OKAY"
    mov rax, 0x1F591F411F4B1F4F
    mov qword[0xB8000], rax
    hlt
