
%include "include.asm"

TEST_TEXT_LEN equ 23

section .data
    filename db "/tmp/http-syscall/write-file.txt",0
    filename2 db "/tmp/http-syscall/write-file2.txt",0
    text db "Here's some test text.", 10

section .bss
    ; write-file.txt
    filefd resq 1
    ; write-file2.txt
    file2fd resq 1
    text_buffer resb 256

section .text
    global _start

; Write the test text to write-file.txt
write_to_write_file:
    ; first open (and create if doesn't exist) the file
    mov rax, SYS_OPEN
    mov rdi, filename ; null terminated string as a filename
    mov rsi, O_CREAT+O_WRONLY ; create and write flags
    mov rdx, 0644o ; file permissions. the o tells nasm that its octal number
    syscall

    ; sys_open returns the file fd or the error to rax
    ; so compare if rax is smaller than 0 (< 0 are errors)
    ; cmp rax, 0

    ; save the opened file's file descriptor
    mov [filefd], rax

    ; Then we write the text varible to the opened file
    mov rax, SYS_WRITE
    mov rdi, [filefd] ; first move the returned fd from rax register
    mov rsi, text
    mov rdx, TEST_TEXT_LEN
    syscall

    ; finally close the file
    mov rax, SYS_CLOSE
    mov rdi, [filefd]
    syscall

    ret

; Read the test text from write-file.txt and write it to write-file2.txt
readwrite_to_write2_file:
    ; first open (and create if doesn't exist) the file
    mov rax, SYS_OPEN
    mov rdi, filename ; null terminated string as a filename
    mov rsi, O_CREAT+O_RDWR ; create and write flags
    mov rdx, 0644o ; file permissions. the o tells nasm that its octal number
    syscall

    ; save the opened file's file descriptor
    mov [filefd], rax

    ; then we can read
    mov rax, SYS_READ ; then move the syscall id to rax
    mov rdi, [filefd]
    mov rsi, text_buffer
    mov rdx, TEST_TEXT_LEN
    syscall

    ; finally close the file
    mov rax, SYS_CLOSE
    mov rdi, [filefd]
    syscall

    ; write to write-file2.txt

    ; first open (and create if doesn't exist) the file
    mov rax, SYS_OPEN
    mov rdi, filename2
    mov rsi, O_CREAT+O_RDWR ; create and write flags
    mov rdx, 0644o ; file permissions. the o tells nasm that its octal number
    syscall

    ; save the opened file's file descriptor
    mov [file2fd], rax

    ; Then we write the text varible to the opened file
    mov rax, SYS_WRITE
    mov rdi, [file2fd] ; first move the returned fd from rax register
    mov rsi, text_buffer
    mov rdx, TEST_TEXT_LEN
    syscall

     ; finally close the file
    mov rax, SYS_CLOSE
    mov rdi, [file2fd]
    syscall

    ret

_start:
    call write_to_write_file
    call readwrite_to_write2_file

    mov rax, SYS_EXIT
    mov rdi, 0
    syscall
