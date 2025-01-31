%include "src/common.s"

extern free
extern realloc

global vec_new
global vec_free
global vec_grow
global vec_push
global vec_pop

%define VEC_MIN_CAPACITY 8

section .text

; void vec_new(vec_t *vec)
vec_new:
    push rbp
    mov rbp, rsp

    mov qword [rdi+vec_t.ptr], 0
    mov qword [rdi+vec_t.len], 0
    mov qword [rdi+vec_t.capacity], 0

    pop rbp
    ret

; void vec_free(vec_t *vec)
vec_free:
    push rbp
    mov rbp, rsp

    mov rdi, qword [rdi+vec_t.ptr]
    cmp rdi, 0
    je _vec_free_done

    call free

_vec_free_done:
    pop rbp
    ret

; void vec_grow(vec_t *vec)
vec_grow:
%define FRAME_SIZE 16
%define self (rbp-FRAME_SIZE)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    mov qword [self], rdi

    mov rsi, qword [rdi+vec_t.capacity]
    cmp rsi, VEC_MIN_CAPACITY
    jl _vec_grow_else
    shl rsi, 1
    jmp _vec_grow_end_if
_vec_grow_else:
    mov rsi, VEC_MIN_CAPACITY
_vec_grow_end_if:
    mov [rdi+vec_t.capacity], rsi

    mov rdi, qword [rdi+vec_t.ptr]
    call realloc

    mov rdi, qword [self]
    mov qword [rdi+vec_t.ptr], rax

    add rsp, FRAME_SIZE
    pop rbp
    ret

; void vec_push(vec_t *vec, long item)
vec_push:
%define FRAME_SIZE 16
%define self (rbp-FRAME_SIZE)
%define item (rbp-FRAME_SIZE+8)
    push rbp
    mov rbp, rsp
    sub rsp, FRAME_SIZE

    mov qword [self], rdi
    mov qword [item], rsi

    mov rdi, qword [rdi+vec_t.capacity]
    mov rsi, qword [rdi+vec_t.len]
    cmp rdi, rsi
    jg _vec_push_after_grow

    call vec_grow

_vec_push_after_grow:
    mov rdi, qword [self]
    mov rsi, qword [item]
    mov rdx, qword [rdi+vec_t.ptr]
    mov rcx, qword [rdi+vec_t.len]

    mov qword [rdx+rcx*vec_item_size], rsi
    inc rcx

    mov qword [rdi+vec_t.len], rcx

    add rsp, FRAME_SIZE
    pop rbp
    ret

; Returns zero if the vector is empty.
; char vec_pop(vec_t *vec, long *item)
vec_pop:
    push rbp
    mov rbp, rsp

    mov rdx, qword [rdi+vec_t.len]
    cmp rdx, 0
    je _vec_pop_empty

    dec rdx
    mov rcx, qword [rdi+vec_t.ptr]
    mov rax, qword [rcx+rdx*vec_item_size]
    mov qword [rsi], rax
    mov qword [rdi+vec_t.len], rdx

    jmp _vec_pop_done

_vec_pop_empty:
    mov rax, 0

_vec_pop_done:
    pop rbp
    ret
