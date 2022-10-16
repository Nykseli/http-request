
section .bss
    print_pointer resb 8

; Print null terminated string
%macro print 1
    mov rax, %1 ; rax char pointer
    mov [print_pointer], rax
    mov rbx, 0 ; rbx str len
; calculate the length of the string
%%len_loop:
    mov cl, [rax] ; get one char at the time. moving to rcx would move 8 characters
    cmp cl, 0
    je %%print_loop_end
    inc rax
    inc rbx
    jmp %%len_loop
; finally print the text
%%print_loop_end:
    mov rax, SYS_WRITE
    mov rdi, STDOUT
    mov rsi, [print_pointer]
    mov rdx, rbx
    syscall
%endmacro

;;;
; stdio
;;
STDIN   equ 0
STDOUT  equ 1
STDERR  equ 2

;;;
; SYS_OPEN options
;;
O_RDONLY    equ 0
O_WRONLY    equ 1
O_RDWR      equ 2
O_CREAT     equ 64
O_EXCL      equ 128
O_APPEND    equ 1024
O_NONBLOCK  equ 2048

;;;
; System calls
;;
SYS_READ  equ 0x0000 ;  0
SYS_WRITE equ 0x0001 ;  1
SYS_OPEN  equ 0x0002 ;  2
SYS_CLOSE equ 0x0003 ;  3
SYS_EXIT  equ 0x003c ; 60

