global loader
global stack_ptr

extern kernel_main

MODULEALIGN equ 1<<0
MEMINFO equ 1<<1
FLAGS equ MODULEALIGN | MEMINFO
MAGIC equ 0x1BADB002
CHECKSUM equ -(MAGIC + FLAGS)

section .mbheader
align 4
MultiBootHeader:
  dd MAGIC
  dd FLAGS
  dd CHECKSUM

section .text

STACKSIZE equ 0x4000

loader:
  mov rsp, stack+STACKSIZE  ; Use rsp for 64-bit mode
  push rax                  ; Use rax for 64-bit mode
  push rbx                  ; Use rbx for 64-bit mode

  call kernel_main
  cli

hang:
  hlt
  jmp hang

section .bss
align 4
stack:
  resb STACKSIZE
stack_ptr:
