BITS 32

global main

main:

do_write:
	mov edx, 24
	call get_data
.do_exit:
	mov ecx, 0
	mov eax, 0
	int 0x80

.got_data:
	pop ecx
	mov ebx, 1
	mov eax, 4
	int 0x80
    call goto_start
    
get_data:
call do_write.got_data
data:
db	"This version is patched",0xa
data_end:

goto_start:
    pop edx
    pop ebx
    db 0xe9
