CC = gcc
DAS = ndisasm
AS = nasm
#AS = as
LD = ld
CFLAGS = -g -fPIC -fno-asynchronous-unwind-tables
SCFLAGS =  -g -fPIC -shared -fno-asynchronous-unwind-tables
LDFLAGS = -lc -ldl
ASFLAGS =

all: ldpreload tests

tests: buffer_overflow int_overflow simple_main

ldpreload: libhooks.so

.PHONY:
libhooks.so: hooks.c
	$(CC) $(SCFLAGS) $^ -o $@ $(LDFLAGS)
	mv libhooks.so ../bin/libhooks.so

.PHONY:
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

.PHONY:
int_overflow: overflow_and_test.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)
	mv int_overflow ../bin/int_overflow

.PHONY:
buffer_overflow: buffer_over.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)
	mv buffer_overflow ../bin/buffer_overflow

.PHONY:
simple_main: main.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)
	mv simple_main ../bin/simple_main

clean:
	rm *.o ../bin/int_overflow ../bin/buffer_overflow ../bin/simple_main ../bin/libhooks.so

