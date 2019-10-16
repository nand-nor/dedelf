#define _GNU_SOURCE

#include <stdio.h>
#include <dlfcn.h>
#include<errno.h>
#include<sys/types.h>
#include<signal.h>
#include<sys/syscall.h>
#include<unistd.h>
#include<stdlib.h>
#include<pthread.h>
#include<linux/kernel.h>
#include<linux/module.h>
#include<linux/unistd.h>
#define LIBC "libc.so.6"

typedef int (*old_libc_start_main)(int *(main) (int, char * *, char * *), 
	int argc, char * * ubp_av, void (*init) (void), void (*fini) (void), 
	void (*rtld_fini) (void), void (* stack_end));

typedef void (*old_libc_csu_fini)(void);

typedef int (*old_execv)(const char *, char *const argv[]);

int COUNT = 1;
void *libc;
void *ld_2;
static int (*return_main)(int, char **, char **);


void __attribute ((constructor)) init(void) {
   libc=dlopen(LIBC, RTLD_LAZY);

}

union Args {
	int argc;
	char **argv; 
	char **env;
};

static int new_main(int argc, char**argv, char**env){
	printf("Pre main code run, changing args to main... >:) %d\n", COUNT);
    
    if(argv) printf("argv: %s\n", *argv);
    if(env) printf("env: %s\n", *env);
    dlclose(libc);

	return return_main(argc, argv, env);

}

int __libc_start_main(int *(main) (int, char * *, char * *), int argc, 
	char * * ubp_av, void (*init) (void), void (*fini) (void), 
	void (*rtld_fini) (void), void (* stack_end)) {
 	 ++COUNT;

	system("notify-send oi-oi-oi whats-up");

    if(!libc){
        libc = dlopen(LIBC, RTLD_LAZY);
    }
    old_libc_start_main old_start;
    old_start = dlsym(libc,"__libc_start_main");
    printf("Hoooked __libc_start_main confirmation %d\n", argc);
    return_main = main;
    return old_start(new_main, argc, ubp_av, init, fini, rtld_fini, stack_end);
}


void __libc_csu_fini(void){
    if(!libc){
        libc = dlopen(LIBC, RTLD_LAZY);
    }
    
    old_libc_csu_fini old_end;
    old_end = dlsym(libc,"__libc_csu_fini");
    printf("Hoooked __libc_csu_end confirmation. what would you like to do?\n");
   
    dlclose(libc);
    execl("/bin/sh", "sh", "-c", "notify-send hey-evan whats-up", (char *)NULL);
}


