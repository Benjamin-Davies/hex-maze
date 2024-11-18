%include "src/common.s"

extern printf

extern term_init
extern term_exit
extern term_should_exit
extern term_poll
extern term_read
extern term_flush
extern term_clear
extern term_goto

global main

section .data
hello db `Hello, World!`, 0

section .text
main:
%define FRAME_SIZE 16
%define timeout (rbp-FRAME_SIZE)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    call term_init

main_loop:
    call term_clear
    mov rdi, 0
    mov rsi, 0
    call term_goto

    mov rdi, hello
    call printf

    call term_flush

    mov dword [timeout], 16
input_loop:
    mov edi, dword [timeout]
    call term_poll
    cmp eax, 0
    jle input_loop_end

    call term_read
    cmp rax, CTRL_C
    je main_loop_end
    cmp rax, ESC
    je main_loop_end
    cmp rax, "q"
    je main_loop_end

    mov dword [timeout], 0
    jmp input_loop
input_loop_end:

    mov eax, dword [term_should_exit]
    cmp eax, 0
    jne main_loop_end

    jmp main_loop
main_loop_end:

    call term_exit

    mov eax, 0
    add rsp, FRAME_SIZE
    pop rbp
    ret
