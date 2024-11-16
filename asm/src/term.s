extern memcpy

extern fflush
extern printf
extern stdout

extern cfmakeraw
extern tcgetattr
extern tcsetattr

global term_init
global term_exit
global term_flush
global term_clear
global term_goto

%define TCSADRAIN 1

struc termios
    .c_iflag resd 1
    .c_oflag resd 1
    .c_cflag resd 1
    .c_lflag resd 1
    .c_line resb 1
    .c_cc resb 32
    _padding resb 3
    .c_ispeed resd 1
    .c_ospeed resd 1
endstruc

section .data
%define CSI 0x1b, "["

enable_alt_screen db CSI, "?1049h", 0
disable_alt_screen db CSI, "?1049l", 0
show_cursor db CSI, "?25h", 0
hide_cursor db CSI, "?25l", 0

clear_screen db CSI, "2J", 0
goto db CSI, "%d;%dH", 0

section .bss
old_termios:
    istruc termios iend

section .text
term_init:
    push rbp
    mov rbp, rsp
    sub rsp, termios_size+4

    ; init termios
    mov rdi, 0
    lea rsi, [rbp-termios_size]
    call tcgetattr

    lea rdi, [old_termios]
    lea rsi, [rbp-termios_size]
    mov rdx, termios_size
    call memcpy

    lea rdi, [rbp-termios_size]
    call cfmakeraw

    mov rdi, 0
    mov rsi, TCSADRAIN
    lea rdx, [rbp-termios_size]
    call tcsetattr

    ; init CSI
    lea rdi, [enable_alt_screen]
    call printf
    lea rdi, [hide_cursor]
    call printf
    call term_flush

    add rsp, termios_size+4
    pop rbp
    ret

term_exit:
    push rbp

    ; restore CSI
    lea rdi, [disable_alt_screen]
    call printf
    lea rdi, [show_cursor]
    call printf
    call term_flush

    ; restore termios
    mov rdi, 0
    mov rsi, TCSADRAIN
    lea rdx, [old_termios]
    call tcsetattr

    pop rbp
    ret

term_flush:
    push rbp

    mov rdi, [stdout]
    call fflush

    pop rbp
    ret

term_clear:
    push rbp

    lea rdi, [clear_screen]
    call printf

    pop rbp
    ret

term_goto:
    push rbp

    mov rdx, rsi
    mov rsi, rdi
    inc rsi
    inc rdx

    lea rdi, [goto]
    call printf

    pop rbp
    ret
