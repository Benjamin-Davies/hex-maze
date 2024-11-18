%include "src/common.s"

extern printf

extern term_flush
extern term_goto
extern term_get_size

global maze_init
global maze_draw

section .data
hello db "hello", 0

section .bss
cols resw 1
rows resw 1

section .text
; void init()
maze_init:
%define FRAME_SIZE 16
%define ws (rbp-8)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    call term_get_size
    mov qword [ws], rax
    mov di, word [ws+winsize_t.ws_col]
    mov si, word [ws+winsize_t.ws_row]

    cmp di, 11
    jl _maze_init_empty
    cmp si, 7
    jl _maze_init_empty

    mov ax, di
    sub ax, 1
    mov dx, 0
    mov di, 4
    div di
    mov word [cols], ax

    mov ax, si
    sub ax, 2
    mov dx, 0
    mov di, 2
    div di
    mov word [rows], ax

    jmp _maze_init_end

_maze_init_empty:
    mov word [cols], word 0
    mov word [rows], word 0

_maze_init_end:
    add rsp, FRAME_SIZE
    pop rbp
    ret

; void draw()
maze_draw:
%define FRAME_SIZE 16
%define height (rbp-FRAME_SIZE)
%define cols_ (rbp-FRAME_SIZE+2)
%define y (rbp-FRAME_SIZE+4)
%define col (rbp-FRAME_SIZE+6)
%define half_row (rbp-FRAME_SIZE+8)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    mov ax, word [rows]
    lea rax, [rax*2+2]
    mov word [height], ax
    mov ax, word [cols]
    mov word [cols_], ax

    mov word [y], 0
_y_loop:
    mov ax, word [y]
    cmp ax, word [height]
    jge _y_loop_end

    mov di, 0
    mov si, word [y]
    call term_goto

    lea rdi, [hello]
    call printf
    call term_flush

    inc word [y]
    jmp _y_loop
_y_loop_end:

    add rsp, FRAME_SIZE
    pop rbp
    ret
