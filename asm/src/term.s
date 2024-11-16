%include "src/common.s"

extern memcpy
extern poll
extern read

extern fflush
extern printf
extern stdout

extern cfmakeraw
extern tcgetattr
extern tcsetattr

global term_init
global term_exit
global term_poll
global term_read
global term_flush
global term_clear
global term_goto

%define POLLIN 1

struc pollfd
    .fd resd 1
    .events resd 1
    .revents resd 1
endstruc

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
%define CSI ESC, "["

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
%define FRAME_SIZE (termios_size+4)
%define current_termios (rbp-FRAME_SIZE)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    ; init termios
    mov rdi, 0
    lea rsi, [current_termios]
    call tcgetattr

    lea rdi, [old_termios]
    lea rsi, [current_termios]
    mov rdx, termios_size
    call memcpy

    lea rdi, [current_termios]
    call cfmakeraw

    mov rdi, 0
    mov rsi, TCSADRAIN
    lea rdx, [current_termios]
    call tcsetattr

    ; init CSI
    lea rdi, [enable_alt_screen]
    call printf
    lea rdi, [hide_cursor]
    call printf
    call term_flush

    add rsp, FRAME_SIZE
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

term_poll:
%define FRAME_SIZE (pollfd_size+4)
%define fds (rbp-FRAME_SIZE)
%define timeout (rbp-FRAME_SIZE+pollfd_size)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE
    mov dword [timeout], edi

    mov dword [fds+pollfd.fd], 0
    mov dword [fds+pollfd.events], POLLIN
    mov dword [fds+pollfd.revents], 0

    lea rdi, [fds]
    mov rsi, 1
    mov edx, dword [timeout]
    call poll

    add rsp, FRAME_SIZE
    pop rbp
    ret

term_read:
%define FRAME_SIZE (16)
%define buffer (rbp-FRAME_SIZE)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE
    xor eax, eax
    mov dword [buffer], eax

    mov rdi, 0
    lea rsi, dword [buffer]
    mov rdx, 1
    call read

    mov eax, dword [buffer]
    add rsp, FRAME_SIZE
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
