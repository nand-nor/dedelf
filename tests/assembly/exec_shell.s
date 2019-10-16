# 1 "exec_shell.S"
# 1 "<built-in>"
# 1 "<command-line>"
# 31 "<command-line>"
# 1 "/usr/include/stdc-predef.h" 1 3 4
# 32 "<command-line>" 2
# 1 "exec_shell.S"
 .file "exec_shell.c"
 .text
 .section .rodata
.LC0:
 .string "sh"
.LC1:
 .string "/bin/sh"
.LC2:
 .string "oi hai"
 .text
 .globl main
 .type main, @function
main:
.LFB0:
 .cfi_startproc
 pushq %rbp
 .cfi_def_cfa_offset 16
 .cfi_offset 6, -16
 movq %rsp, %rbp
 .cfi_def_cfa_register 6
 movl $0, %edi
 call setgid@PLT
 movl $0, %edi
 call setuid@PLT
 movl $0, %ecx
 movl $0, %edx
 leaq .LC0(%rip), %rsi
 leaq .LC1(%rip), %rdi
 movl $0, %eax
 call execl@PLT
 leaq .LC2(%rip), %rdi
 movl $0, %eax
 call printf@PLT
 movl $0, %eax
 popq %rbp
 .cfi_def_cfa 7, 8
 ret
 .cfi_endproc
.LFE0:
 .size main, .-main
 .ident "GCC: (Ubuntu 7.5.0-3ubuntu1~18.04) 7.5.0"
 .section .note.GNU-stack,"",@progbits
