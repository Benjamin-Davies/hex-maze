%include "src/common.s"

global hex_on_grid

section .text
; int hex_on_grid(short col, short half_row)
hex_on_grid:
    ; Short way to calculate (di + si) % 2 == 0
    mov ax, di
    add ax, si
    and eax, 1
    xor eax, 1
    ret
