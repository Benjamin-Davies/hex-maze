%include "src/common.s"

global hex_nearest_north
global hex_on_grid
global hex_ortho_to_stagg

section .text

; Doesn't require 16-byte stack alignment
; int hex_on_grid(short col, short half_row)
hex_on_grid:
    ; Short way to calculate (di + si) % 2 == 0
    mov ax, di
    add ax, si
    and eax, 1
    xor eax, 1
    ret

; Returns the coordinates in the registers where they were passed
; Doesn't require 16-byte stack alignment
; void hex_nearest_north(inout short col, inout short half_row)
hex_nearest_north:
    push rbp
    mov rbp, rsp

    call hex_on_grid
    xor ax, 1
    sub si, ax

    pop rbp
    ret

; Returns the coordinates in the registers where they were passed
; Doesn't require 16-byte stack alignment
; void hex_ortho_to_stagg(inout short col, inout short half_row)
hex_ortho_to_stagg:
    ; let ax = col % 2
    mov ax, 1
    and ax, di
    ; let half_row = half_row - col % 2
    sub si, ax
    ; let row = half_row / 2
    sar si, 1
    ret
