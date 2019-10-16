# Test Files: Description #

The files in this directory are trimmed versions of the binaries found
in the `bin/` and `assembly/` directories. 

1. `inj_bytes_loop` : file size 0x1000 bytes of a simple infinite loop

```
int main(){
    while(1){

    }
}
```
works with `simple_main` when added to end of `.eh_frame` and `.text`.
TODO: investigate more test cases i.e. offsets within the executable
load segment

2. `inj_bytes_loop_small` : file size 0x1c0 bytes of the same simple infinite loop, without the page-aligned padding.





