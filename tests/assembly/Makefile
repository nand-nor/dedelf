DAS = ndisasm
AS = nasm
#AS = as
LD = ld
CFLAGS = -g -fPIC -shared
LDFLAGS = -lc #-ldl
ASFLAGS =
CC = gcc

all: dissassemble nasm

dissassemble: exec_shell.ndisasm

nasm: inject.o inject


exec_shell.ndisasm: exec_shell
	$(DAS) -b 64 $^ > $@

inject.o: inject.asm
	$(AS) -f elf64 $^ -o $@

#inject: inject.o


clean:
	rm *.s *.S *.asm *.ndisasm
