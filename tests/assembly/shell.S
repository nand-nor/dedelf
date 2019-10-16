global _start

extern setgid
extern setuid
extern execl

section .text


_start:
    push    r12
    push    r13
    push    r14
	mov 	edi, 0
	call	setgid
	mov 	edi, 0
	call	setuid
	mov 	ecx, 0
	mov 	edx, 0
	lea	    rsi, [sh]
	lea 	rdi, [binsh]
	mov 	eax, 0
	call	execl
	jmp	    loop

loop:
        jmp loop
sh:
        db	"sh", 0xa
binsh:
        db	"/bin/sh", 0xa
ohai:
        db	"oi hai", 0xa