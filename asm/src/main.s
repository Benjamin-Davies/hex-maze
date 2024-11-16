extern printf
extern sleep

extern term_init
extern term_exit
extern term_flush
extern term_clear
extern term_goto

global main

section .data
hello db `Hello, World!`, 0
decimal db `%d`, 0

section .bss
test:
    resb 1

section .text
main:
    push rbp

    call term_init

    call term_clear
    mov rdi, 0
    mov rsi, 0
    call term_goto

    mov rdi, hello
    call printf

    call term_flush

    mov rdi, 1
    call sleep

    call term_exit

    mov eax, 0
    pop rbp
    ret
