%include "src/common.s"

extern calloc
extern free

extern hex_ortho_to_stagg

global grid_new
global grid_free
global grid_get

section .text

; void grid_new(grid_t *grid, short cols, short rows)
grid_new:
%define FRAME_SIZE 16
%define grid (rbp-FRAME_SIZE) ; grid_t *grid
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    mov qword [grid], rdi
    mov word [rdi+grid_t.cols], si
    mov word [rdi+grid_t.rows], dx

    ; let rax = cols * rows
    movzx rax, si
    movzx rdx, dx
    mul rdi

    mov rdi, rax
    mov rsi, 8
    call calloc

    mov rdi, qword [grid]
    mov qword [rdi+grid_t.ptr], rax

    add rsp, FRAME_SIZE
    pop rbp
    ret

; void grid_free(grid_t *grid)
grid_free:
    push rbp
    mov rbp, rsp

    mov rdi, qword [rdi+grid_t.ptr]
    call free

    pop rbp
    ret

; long *grid_get(grid_t *grid, short col, short half_row)
grid_get:
    push rbp
    mov rbp, rsp

    mov rcx, rdi
    mov di, si
    mov si, dx
    call hex_ortho_to_stagg

    movzx rax, si
    movzx rdx, word [rcx+grid_t.cols]
    mul rdx
    movzx rdi, di
    add rax, rdi

    mov rdi, qword [rcx+grid_t.ptr]
    lea rax, qword [rdi+rax*grid_item_size]

    pop rbp
    ret
