#include <stdio.h>

int and_func(__uint16_t x, __uint16_t y){
    int z = 0;
    z = x & y;
    return z;
}

//patch args to hold overflow bytes
int main(void){
    int anded = and_func(10,12);
    printf("Anded is %d", anded);
    return 0;
}

