%define STDIN 0
%define STDOUT 1
%define STDERR 2

%define CTRL_C 0x03
%define ESC 0x1B

struc winsize_t
    .ws_row resw 1
    .ws_col resw 1
    .ws_xpixel resw 1
    .ws_ypixel resw 1
endstruc

struc grid_t
    .ptr resq 1
    .cols resw 1
    .rows resw 1
endstruc
%define grid_item_size 8

struc vec_t
    .ptr resq 1
    .len resq 1
    .capacity resq 1
endstruc
%define vec_item_size 8
