%include "src/common.s"

extern printf
extern putchar

extern grid_new
extern grid_get

extern hex_nearest_north
extern hex_on_grid
extern hex_ortho_to_stagg

extern term_flush
extern term_goto
extern term_get_size

global maze_init
global maze_wall_between
global maze_wall_between_ptr
global maze_draw
global maze_cells

struc cell_t
    .north_east resb 1
    .south resb 1
    .north_west resb 1
    .background resb 1
endstruc

section .data

hwall_none db "   ", 0
hwall_some db "___", 0

section .bss

maze_cells:
    istruc grid_t
        at .ptr, resq 1
        at .cols, resw 1
        at .rows, resw 1
    iend
%define cols (maze_cells+grid_t.cols)
%define rows (maze_cells+grid_t.rows)

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

    lea rdi, [maze_cells]
    mov si, word [cols]
    mov dx, word [rows]
    call grid_new

    movzx rax, word [cols]
    movzx rdi, word [rows]
    mul rdi ; rax = cols * rows
    mov rdi, qword [maze_cells+grid_t.ptr] ; start_ptr
    mov rsi, 0 ; index
_maze_init_loop:
    cmp rsi, rax
    jge _maze_init_loop_end

    lea rdx, qword [rdi+rsi*grid_item_size] ; current_ptr
    mov byte [rdx+cell_t.north_east], 1
    mov byte [rdx+cell_t.south], 1
    mov byte [rdx+cell_t.north_west], 1

    inc rsi
    jmp _maze_init_loop
_maze_init_loop_end:

    jmp _maze_init_end

_maze_init_empty:
    mov word [cols], word 0
    mov word [rows], word 0

_maze_init_end:
    add rsp, FRAME_SIZE
    pop rbp
    ret

; Doesn't require 16-byte stack alignment
; char contains(short col, short half_row)
maze_contains:
    call hex_ortho_to_stagg

    cmp di, 0
    jnge _maze_doesnt_contain
    cmp di, [cols]
    jnl _maze_doesnt_contain
    cmp si, 0
    jnge _maze_doesnt_contain
    cmp si, [rows]
    jnl _maze_doesnt_contain

_maze_does_contain:
    mov eax, 1
    ret

_maze_doesnt_contain:
    mov eax, 0
    ret

; char maze_wall_between(short a_col, short a_half_row, short b_col, short b_half_row)
maze_wall_between:
%define FRAME_SIZE 16
%define a_col (rbp-FRAME_SIZE) ; short
%define a_half_row (rbp-FRAME_SIZE+2) ; short
%define b_col (rbp-FRAME_SIZE+4) ; short
%define b_half_row (rbp-FRAME_SIZE+6) ; short
%define a_inside (rbp-FRAME_SIZE+8) ; char
%define b_inside (rbp-FRAME_SIZE+9) ; char
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE
    mov word [a_col], di
    mov word [a_half_row], si
    mov word [b_col], dx
    mov word [b_half_row], cx

    call maze_contains
    mov byte [a_inside], al
    mov di, word [b_col]
    mov si, word [b_half_row]
    call maze_contains
    mov byte [b_inside], al

    ; !(a_inside || b_inside)
    mov al, [a_inside]
    or al, [b_inside]
    cmp al, 0
    je _maze_has_no_wall_between

    ; !a_inside && b_inside
    mov al, [a_inside]
    xor al, 1
    and al, [b_inside]
    cmp al, 0
    jne _maze_has_wall_between

    ; a_inside && !b_inside
    mov al, [b_inside]
    xor al, 1
    and al, [a_inside]
    cmp al, 0
    jne _maze_has_wall_between

    mov di, word [a_col]
    mov si, word [a_half_row]
    mov dx, word [b_col]
    mov cx, word [b_half_row]
    call maze_wall_between_ptr
    mov al, byte [rax]
    jmp _maze_wall_between_end

_maze_has_no_wall_between:
    mov al, 0
    jmp _maze_wall_between_end

_maze_has_wall_between:
    mov al, 1

_maze_wall_between_end:
    add rsp, FRAME_SIZE
    pop rbp
    ret

; char *maze_wall_between_ptr(short a_col, short a_half_row, short b_col, short b_half_row)
maze_wall_between_ptr:
%define FRAME_SIZE 16
%define a_col (rbp-FRAME_SIZE) ; short
%define a_half_row (rbp-FRAME_SIZE+2) ; short
%define b_col (rbp-FRAME_SIZE+4) ; short
%define b_half_row (rbp-FRAME_SIZE+6) ; short
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE
    mov word [a_col], di
    mov word [a_half_row], si
    mov word [b_col], dx
    mov word [b_half_row], cx

    ; let delta_col = b_col - a_col
    movzx rdx, word [b_col]
    movzx rdi, word [a_col]
    sub rdx, rdi
    ; let delta_half_row = b_half_row - a_half_row
    movzx rcx, word [b_half_row]
    movzx rsi, word [a_half_row]
    sub rcx, rsi
    ; let mask = is_negative(delta_half_row)<<2 + delta_col&3
    and rcx, 4
    and rdx, 3
    or rdx, rcx
    ; jmp _maze_wall_jump_table[mask]
    mov rax, qword [_maze_wall_jump_table+8*rdx]
    jmp rax

    align 8
_maze_wall_jump_table dq \
    _maze_wall_south, _maze_wall_south_east, \
    _maze_wall_south, _maze_wall_south_west, \
    _maze_wall_north, _maze_wall_north_east, \
    _maze_wall_north, _maze_wall_north_west

_maze_wall_north:
    mov si, word [b_col]
    mov dx, word [b_half_row]
    jmp _maze_wall_n_or_s
_maze_wall_north_east:
    mov si, word [a_col]
    mov dx, word [a_half_row]
    jmp _maze_wall_ne_or_sw
_maze_wall_south_east:
    mov si, word [b_col]
    mov dx, word [b_half_row]
    jmp _maze_wall_nw_or_se
_maze_wall_south:
    mov si, word [a_col]
    mov dx, word [a_half_row]
    jmp _maze_wall_n_or_s
_maze_wall_south_west:
    mov si, word [b_col]
    mov dx, word [b_half_row]
    jmp _maze_wall_ne_or_sw
_maze_wall_north_west:
    mov si, word [a_col]
    mov dx, word [a_half_row]
    jmp _maze_wall_nw_or_se

_maze_wall_n_or_s:
    lea rdi, [maze_cells]
    call grid_get
    lea rax, byte [rax+cell_t.south]
    jmp _maze_wall_ptr_end
_maze_wall_ne_or_sw:
    lea rdi, [maze_cells]
    call grid_get
    lea rax, byte [rax+cell_t.north_east]
    jmp _maze_wall_ptr_end
_maze_wall_nw_or_se:
    lea rdi, [maze_cells]
    call grid_get
    lea rax, byte [rax+cell_t.north_west]

_maze_wall_ptr_end:
    add rsp, FRAME_SIZE
    pop rbp
    ret

; char maze_vertical_wall_at(short col, short half_row)
maze_horizontal_wall_at:
    push rbp
    mov rbp, rsp

    call hex_nearest_north

    mov dx, di
    mov cx, si
    add cx, 2
    call maze_wall_between

    pop rbp
    ret

; char maze_vertical_wall_at(short col, short half_row)
maze_vertical_wall_at:
%define FRAME_SIZE 16
%define col (rbp-FRAME_SIZE) ; short
%define half_row (rbp-FRAME_SIZE+2) ; short
    push rbp
    mov rbp, rsp
    mov word [col], di
    mov word [half_row], si

    call hex_on_grid
    cmp al, 0
    je _maze_vwall_even
_maze_vwall_odd:
    mov di, word [col]
    dec di
    mov si, word [half_row]
    dec si
    mov dx, word [col]
    mov cx, word [half_row]
    jmp _maze_vwall_end

_maze_vwall_even:
    mov di, word [col]
    dec di
    mov si, word [half_row]
    inc si
    mov dx, word [col]
    mov cx, word [half_row]

_maze_vwall_end:
    call maze_wall_between

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

    mov ax, word [y]
    dec ax
    mov word [half_row], ax

    mov di, 0
    mov si, word [y]
    call term_goto

    mov word [col], 0
_col_loop:
    mov ax, word [col]
    cmp ax, word [cols_]
    jge _col_loop_end

    mov di, word [col]
    mov si, word [half_row]
    call maze_vertical_wall_at
    cmp al, 0
    je _vwall_missing
    mov di, word [col]
    mov si, word [half_row]
    call hex_on_grid
    cmp al, 0
    je _vwall_backward
_vwall_forward:
    mov edi, '/'
    jmp _vwall_type_end
_vwall_backward:
    mov edi, '\'
    jmp _vwall_type_end
_vwall_missing:
    mov edi, ' '
_vwall_type_end:
    call putchar

    mov di, word [col]
    mov si, word [half_row]
    call maze_horizontal_wall_at
    cmp al, 0
    je _hwall_none
    mov di, word [col]
    mov si, word [half_row]
    call hex_on_grid
    cmp eax, 0
    jne _hwall_none
_hwall_some:
    mov rdi, hwall_some
    jmp _hwall_end
_hwall_none:
    mov rdi, hwall_none
_hwall_end:
    call printf

    inc word [col]
    jmp _col_loop
_col_loop_end:

    mov di, word [cols]
    mov si, word [half_row]
    call maze_vertical_wall_at
    cmp al, 0
    je _last_vwall_missing
    mov di, word [cols]
    mov si, word [half_row]
    call hex_on_grid
    cmp al, 0
    je _last_vwall_backward
_last_vwall_forward:
    mov edi, '/'
    jmp _last_vwall_type_end
_last_vwall_backward:
    mov edi, '\'
    jmp _last_vwall_type_end
_last_vwall_missing:
    mov edi, ' '
_last_vwall_type_end:
    call putchar

    inc word [y]
    jmp _y_loop
_y_loop_end:

    add rsp, FRAME_SIZE
    pop rbp
    ret
