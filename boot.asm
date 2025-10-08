; boot.asm - Simple bootloader for tiny-os
; This code runs in 16-bit real mode

[org 0x7c00]        ; BIOS loads bootloader at address 0x7c00

    mov ah, 0x0e    ; BIOS teletype output
    
    ; Print our message
    mov al, 'H'
    int 0x10
    mov al, 'e'
    int 0x10
    mov al, 'l'
    int 0x10
    mov al, 'l'
    int 0x10
    mov al, 'o'
    int 0x10
    mov al, ' '
    int 0x10
    mov al, 'f'
    int 0x10
    mov al, 'r'
    int 0x10
    mov al, 'o'
    int 0x10
    mov al, 'm'
    int 0x10
    mov al, ' '
    int 0x10
    mov al, 't'
    int 0x10
    mov al, 'i'
    int 0x10
    mov al, 'n'
    int 0x10
    mov al, 'y'
    int 0x10
    mov al, '-'
    int 0x10
    mov al, 'o'
    int 0x10
    mov al, 's'
    int 0x10
    mov al, '!'
    int 0x10
    
    ; Infinite loop
    jmp $

    ; Pad to 510 bytes and add boot signature
    times 510-($-$$) db 0
    dw 0xaa55       ; Boot signature
