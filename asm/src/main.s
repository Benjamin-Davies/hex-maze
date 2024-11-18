%include "src/common.s"

extern maze_init
extern maze_draw

extern term_init
extern term_exit
extern term_should_exit
extern term_poll
extern term_read
extern term_flush
extern term_clear
extern term_goto

global main

section .text
; int main()
main:
%define FRAME_SIZE 16
%define timeout (rbp-FRAME_SIZE)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    call term_init
    call term_clear

    call maze_init

_main_loop:
    call maze_draw

    call term_flush

    mov dword [timeout], 16
_input_loop:
    mov edi, dword [timeout]
    call term_poll
    cmp eax, 0
    jle _input_loop_end

    call term_read
    cmp al, CTRL_C
    je _main_loop_end
    cmp al, ESC
    je _main_loop_end
    cmp al, 'q'
    je _main_loop_end
    cmp al, 'r'
    je _input_switch_reset
    jmp _input_switch_end
_input_switch_reset:
    call maze_init
    jmp _input_switch_end
_input_switch_end:

    mov dword [timeout], 0
    jmp _input_loop
_input_loop_end:

    mov eax, dword [term_should_exit]
    cmp eax, 0
    jne _main_loop_end

    jmp _main_loop
_main_loop_end:

    call term_exit

    mov eax, 0
    add rsp, FRAME_SIZE
    pop rbp
    ret
