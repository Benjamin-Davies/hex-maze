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
