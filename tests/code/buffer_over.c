#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>


int main(int argc, char **argv, char **env){
    __int64_t arg1 = strtol(argv[1], "", 10);
    __int64_t arg2 = strtol(argv[2], "", 16);

    check:
        if (arg1) {
            goto check;
        }

    return 0xbadbeef;
}