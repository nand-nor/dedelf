CC = gcc
DAS = ndisasm
AS = nasm
#AS = as
LD = ld
CFLAGS = -g -fPIC -shared
LDFLAGS = -lc #-ldl
ASFLAGS =


all: nasm tests ldpreload dissassemble

nasm:
	make -C assembly

tests:
	make -C code tests

ldpreload:
	make -C code ldpreload

dissassemble:
	make -C assembly dissassemble

clean:
	make -C code clean
	make -C assembly clean


