%include "src/common.s"

extern random
extern srandom
extern time

extern grid_new
extern grid_get

extern hex_ortho_to_stagg

extern maze_cells
extern maze_wall_between_ptr

extern vec_new
extern vec_pop
extern vec_push

global generator_init
global generator_step
global generator_is_done

section .bss

generator_is_done resb 1
_padding resb 3

head:
head_col resw 1
head_half_row resw 1

tail:
    istruc vec_t
        at .ptr, resq 1
        at .len, resq 1
        at .capacity, resq 1
    iend

visited:
    istruc grid_t
        at .ptr, resq 1
        at .cols, resw 1
        at .rows, resw 1
    iend

section .text

; void generator_init()
generator_init:
    push rbp
    mov rbp, rsp

    mov rdi, 0
    call time
    mov rdi, rax
    call srandom

    mov byte [generator_is_done], 0
    mov word [head_col], 0
    mov word [head_half_row], 0

    lea rdi, [tail]
    call vec_new

    lea rdi, [visited]
    mov rsi, [maze_cells+grid_t.cols]
    mov rdx, [maze_cells+grid_t.rows]
    call grid_new

    pop rbp
    ret

; void generator_step()
generator_step:
%define FRAME_SIZE 16
%define next (rbp-FRAME_SIZE)
%define next_col (next+0)
%define next_half_row (next+2)
    push rbp
    mov rbp, rsp

    lea rdi, [visited]
    mov si, word [head_col]
    mov dx, word [head_half_row]
    call grid_get
    mov byte [rax], 1

    lea rdi, qword [next]
    call pick_next_cell
    cmp al, 0
    je _no_next_cell

    mov di, word [head_col]
    mov si, word [head_half_row]
    mov dx, word [next_col]
    mov cx, word [next_half_row]
    call maze_wall_between_ptr
    mov byte [rax], 0

    lea rdi, [tail]
    mov esi, dword [head]
    call vec_push

    mov edi, dword [next]
    mov dword [head], edi

    jmp _update_colors

_no_next_cell:
    lea rdi, [tail]
    lea rsi, [head]
    call vec_pop
    cmp al, 0
    jne _update_colors

_no_tail:
    mov byte [generator_is_done], 1

_update_colors:
    call update_colors

    pop rbp
    ret

; void pick_next_cell(int *next_ptr)
pick_next_cell:
%define FRAME_SIZE 16
%define next_ptr (rbp-FRAME_SIZE)
%define next_col (rbp-FRAME_SIZE+8)
%define next_half_row (rbp-FRAME_SIZE+10)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE
    mov qword [next_ptr], rdi

_pick_next_cell_loop:
    call random
    and rax, 7

    cmp rax, 6
    jge _pick_next_cell_loop

    mov di, word [head_col]
    mov si, word [head_half_row]
    mov dx, word [_directions+4*rax]
    mov cx, word [_directions+4*rax+2]
    add di, dx
    add si, cx
    mov word [next_col], di
    mov word [next_half_row], si

    call hex_ortho_to_stagg
    mov dx, word [visited+grid_t.cols]
    mov cx, word [visited+grid_t.rows]
    cmp di, 0
    jl _pick_next_cell_loop
    cmp di, dx
    jge _pick_next_cell_loop
    cmp si, 0
    jl _pick_next_cell_loop
    cmp si, cx
    jge _pick_next_cell_loop

    lea rdi, [visited]
    mov si, word [next_col]
    mov dx, word [next_half_row]
    call grid_get

    mov al, byte [rax]
    cmp al, 0
    jne _pick_next_cell_loop

    mov rax, [next_ptr]
    mov di, word [next_col]
    mov si, word [next_half_row]
    mov word [rax], di
    mov word [rax+2], si

    add rsp, FRAME_SIZE
    pop rbp
    ret

_directions:
    dw 1, -1
    dw 2, 0
    dw 1, 1
    dw -1, 1
    dw -2, 0
    dw -1, -1

update_colors:
    ret
