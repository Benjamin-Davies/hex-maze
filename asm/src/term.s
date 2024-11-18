%include "src/common.s"

extern memcpy
extern memset
extern poll
extern read
extern sigaction

extern fflush
extern printf
extern stdout

extern cfmakeraw
extern tcgetattr
extern tcsetattr

global term_init
global term_exit
global term_should_exit
global term_poll
global term_read
global term_flush
global term_clear
global term_goto

%define POLLIN 1

struc pollfd_t
    .fd resd 1
    .events resd 1
    .revents resd 1
endstruc

%define SIGTERM 15

struc sigaction_t
    .sa_handler resq 1
    .sa_mask resq 16
    .sa_flags resd 1
    ._padding resd 1
    .sa_restorer resq 1
endstruc

%define TCSADRAIN 1

struc termios_t
    .c_iflag resd 1
    .c_oflag resd 1
    .c_cflag resd 1
    .c_lflag resd 1
    .c_line resb 1
    .c_cc resb 32
    ._padding resb 3
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
    istruc termios_t
        at .c_iflag, resd 1
        at .c_oflag, resd 1
        at .c_cflag, resd 1
        at .c_lflag, resd 1
        at .c_line, resb 1
        at .c_cc, resb 32
        at ._padding, resb 3
        at .c_ispeed, resd 1
        at .c_ospeed, resd 1
    iend
term_should_exit resd 1

section .text
term_init:
%define FRAME_SIZE (sigaction_t_size+termios_t_size+12)
%define action (rbp-FRAME_SIZE)
%define termios (rbp-FRAME_SIZE+sigaction_t_size)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    ; init signals
    mov dword [term_should_exit], 0

    lea rdi, dword [action]
    mov rsi, 0
    mov rdx, sigaction_t_size
    call memset
    mov qword [action+sigaction_t.sa_handler], sig_handler

    mov edi, SIGTERM
    lea rsi, [action]
    mov rdx, 0
    call sigaction

    ; init termios
    mov edi, STDIN
    lea rsi, [termios]
    call tcgetattr

    lea rdi, [old_termios]
    lea rsi, [termios]
    mov rdx, termios_t_size
    call memcpy

    lea rdi, [termios]
    call cfmakeraw

    mov edi, STDIN
    mov esi, TCSADRAIN
    lea rdx, [termios]
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

sig_handler:
    mov eax, 1
    mov dword [term_should_exit], eax
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
    mov rdi, STDIN
    mov rsi, TCSADRAIN
    lea rdx, [old_termios]
    call tcsetattr

    pop rbp
    ret

term_poll:
%define FRAME_SIZE (pollfd_t_size+4)
%define fds (rbp-FRAME_SIZE)
%define timeout (rbp-FRAME_SIZE+pollfd_t_size)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE
    mov dword [timeout], edi

    mov dword [fds+pollfd_t.fd], STDIN
    mov dword [fds+pollfd_t.events], POLLIN
    mov dword [fds+pollfd_t.revents], 0

    lea rdi, [fds]
    mov esi, 1
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
    mov dword [buffer], 0

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
