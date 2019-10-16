BITS 64

global main

main:

do_write:
	mov rdx, 24
	call get_data
.got_data:
	pop rcx
	mov rbx, 1
	mov rax, 4
	int 0x80
    call goto_start    
   
.do_exit:
	mov rcx, 0
	mov rax, 0
	int 0x80

get_data:
call do_write.got_data
data:
db	"This version is patched",0xa
data_end:

goto_start:
    pop rdx
    pop rbx
    db 0xe9