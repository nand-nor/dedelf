; Disassembly of file: inject.obj
; Wed Apr 17 13:41:39 2019
; Mode: 32 bits
; Syntax: YASM/NASM
; Instruction set: 80386


global main
global __x86.get_pc_thunk.ax

extern syscall                                          ; near
extern _GLOBAL_OFFSET_TABLE_                            ; byte


SECTION .text                             ; section number 1, code

main:   ; Function begin
        lea     ecx, [esp+4H]                           ; 0000 _ 8D. 4C 24, 04
        and     esp, 0FFFFFFF0H                         ; 0004 _ 83. E4, F0
        push    dword [ecx-4H]                          ; 0007 _ FF. 71, FC
        push    ebp                                     ; 000A _ 55
        mov     ebp, esp                                ; 000B _ 89. E5
        push    ebx                                     ; 000D _ 53
        push    ecx                                     ; 000E _ 51
        sub     esp, 16                                 ; 000F _ 83. EC, 10
        call    __x86.get_pc_thunk.ax                   ; 0012 _ E8, FFFFFFFC(rel)
        add     eax, _GLOBAL_OFFSET_TABLE_-$            ; 0017 _ 05, 00000001(GOT r)
        lea     edx, [?_001+eax]                        ; 001C _ 8D. 90, 00000000(GOT)
        mov     dword [ebp-0CH], edx                    ; 0022 _ 89. 55, F4
        push    1                                       ; 0025 _ 6A, 01
        lea     edx, [?_002+eax]                        ; 0027 _ 8D. 90, 00000002(GOT)
        push    edx                                     ; 002D _ 52
        push    1                                       ; 002E _ 6A, 01
        push    4                                       ; 0030 _ 6A, 04
        mov     ebx, eax                                ; 0032 _ 89. C3
        call    syscall                                 ; 0034 _ E8, FFFFFFFC(PLT r)
        add     esp, 16                                 ; 0039 _ 83. C4, 10
        mov     eax, 0                                  ; 003C _ B8, 00000000
        lea     esp, [ebp-8H]                           ; 0041 _ 8D. 65, F8
        pop     ecx                                     ; 0044 _ 59
        pop     ebx                                     ; 0045 _ 5B
        pop     ebp                                     ; 0046 _ 5D
        lea     esp, [ecx-4H]                           ; 0047 _ 8D. 61, FC
        ret                                             ; 004A _ C3
; main End of function


SECTION .data                           ; section number 2, data


SECTION .bss                            ; section number 3, bss


SECTION .rodata                         ; section number 4, const

?_001:                                                  ; byte
        db 41H, 00H                                     ; 0000 _ A.

?_002:                                                  ; byte
        db 61H, 00H                                     ; 0002 _ a.

message:
        db "Heyyy whats up helllloooooo", 10

SECTION .text.__x86.get_pc_thunk.ax       ; section number 5, code

__x86.get_pc_thunk.ax:; Function begin
        mov     eax, dword [esp]                        ; 0000 _ 8B. 04 24
        ret                                             ; 0003 _ C3
; __x86.get_pc_thunk.ax End of function


