# DEDelf: Deep elf EDitor #

### NOTE: DEDelf is a Work-In-Progress. See below for current working status ##

## Description ##

DEDelf is a Rust-based ELF utility similar to `elfedit`, but with more options. 
Use DEDelf to:

    1. Modify and Add ELF headers:
        - Modify any executive header field
        - Modify any field in a specified section header
        - Modify any field in a specified program header
        - Add new sections / section headers
        - Add new segments / program headers
    2. Patch/Inject bytes within existing sections/segments
        - Patch additional bytes into to existing segments
        - Overwrite/Replace bytes within existing segments

## Command Line Useage ##

### Injection Mode ###

To specify injection mode, use `-i` or `--inject`, followed by `path/to/infile` `/path/to/injection_bytes`. 
Optionally provide: 
1. `-p <name>`   : section name to place the bytes at the end of (default section is the `.text` section, assuming the infile has such a section. If not then the bytes will be ? (TODO))
2. `-b <offset>` : byte offset to inject bytes at (cannot be specified with `-p` option)
3. `-s <size>`   : page-aligned size (in hex) of injection bytes (default size is 0x1000)
4. `-e <entry>`  : new entry point (in hex) to modify the exec header with (default is no entry modification)
5. `-r`          : replace the bytes of the entire section, rather than appending injection bytes to the end of it.
6. `-o <outfile>`: filename to write modified bytes to (default is to copy the infile name and append `_inj` to the string).


### Modification Mode ###

To specify modification mode, use `-m` or `--modify`, followed by a file to modify, a valid modification type, 
a valid field, a valid replacement value, and others as needed. Valid modification types are as follows:

#### Executive header modifications ####

Supply `exec_header` as the modification type: requires valid executive header field and a valid replacement. 
All numeric values, including offsets and entry points  as well as setion header numbers etc., 
should be in base 16. For a list of valid executive header fields, use the exact field name from 
the executive header struct:
```
        typedef struct {
               unsigned char e_ident[EI_NIDENT];
               uint16_t      e_type;
               uint16_t      e_machine;
               uint32_t      e_version;
               ElfN_Addr     e_entry;
               ElfN_Off      e_phoff;
               ElfN_Off      e_shoff;
               uint32_t      e_flags;
               uint16_t      e_ehsize;
               uint16_t      e_phentsize;
               uint16_t      e_phnum;
               uint16_t      e_shentsize;
               uint16_t      e_shnum;
               uint16_t      e_shstrndx;
           } ElfN_Ehdr;
```

For modifications within the `e_ident` field, only certain replacements are possible, such as changing the ELF 
class, data, and osabi fields. Such modifications require providing the offset constant within the `e_ident` field 
array, rather than the generic field as is the case for all other executive header modifications. For valid offsets and
replacements, use the values defined in ELF specification as found in `elf(5)` (or `elf.h` for full list of options). 

For example, the following commandline options for modification are valid:

 ```
 dedelf -m path/to/infile exec_header EI_CLASS ELFCLASS32
 dedelf -m path/to/infile exec_header EI_DATA ELFDATA2MSB
 dedelf -m path/to/infile exec_header e_shoff 0x40100
```

The first command changes the file's class to 32 bits. The second changes the files data to big endian. The last command
changes the offset of the section header table to byte offset 0x40100. This will not only change the field in the
header, but it will also cause the section header table to be written at that byte offset when the modified contents
are written to a file. In all three cases, no outfile was specified (with the `-o` option) and so the modified bytes
will be written to `path/to/infile_inj`. 


#### Section header modifications ####

Supply `sec_header` as the modification type : requires valid section header field (as seen below), either a section name or section header
 table index, and valid replacement consistent with the field type. Valid constants should be provided as defined in `elf.h`.
For fields that are not constrained by defined constants, provide all numeric values in base 16.             
```
        typedef struct {
               uint32_t     sh_name;
               uint32_t     sh_type;
               uintN_t      sh_flags;
               ElfN_Addr    sh_addr;
               ElfN_Off     sh_offset;
               uintN_t      sh_size;
               uint32_t     sh_link;
               uint32_t     sh_info;
               uintN_t      sh_addralign;
               uintN_t      sh_entsize;
           } ElfN_Shdr;
```

For example, the following args are both valid and will perform the same modification or changing the section 
type of the `.text` section to type `SHT_RELA`:
```
dedelf -m path/to/infile sec_header .text sh_type SHT_RELA
dedelf -m path/to/infile sec_header 0xa sh_type SHT_RELA
``` 

where 0xa is the index of the `.text` section within the section header table. 
Indexes and section names can be obtained from the output of `readelf` 


#### Program header modifications ####

Supply `prog_header` as the modification type : requires valid program header field (as seen below), a program header table index, and a 
valid replacement consistent with the field type. Valid constants should be provided as defined in `elf.h`.
Program header table indices can be obtained from the output of `readelf` .
For fields that are not constrained by defined constants, provide all numeric values in base 16.             
For updating flags, enter a value that corresponds with a bit flag representing all desired flags. For example,
if updating a `p_flags` field to be readable (0x1), writable (0x2), and executable (0x4), enter 0x7.

```
        typedef struct {
               uint32_t     p_type;
               uint32_t     p_flags;
               ElfN_Off     p_offset;
               ElfN_Addr    p_vaddr;
               ElfN_Addr    p_paddr;
               uintN_t      p_filesz;
               uintN_t      p_memsz;
               uintN_t      p_align;
           } ElfN_Phdr;
```

For example, the p_type field is defined by constants. To change the 3rd segment in a file to a loadable segment, use 
the following command:
```
dedelf -m path/to/infile prog_header 0x2 p_type PT_LOAD
```



To write the contents to a desired file name, append `-o path/to/outfile`.


`new_sec` : Not yet implemented. Add new section with corresponding entry in the section header table
Restricted to config file mode.

`new_seg` : Not yet implemented. Add new segment with corresponging entry in the program header table
Restricted to config file mode.

Optionally, a file to write modifications to may be provided as such:
 `-o <outfile>`: filename to write modified bytes to (default is to copy the infile name and append `_inj` to the string).


## Config file useage ##

NOTE: This functionality does not yet exist, but eventually there will be a .json parser that can pull all of the configuration details and populate the required ops fields. 

Specify the use of a config file with the `-f` or `--file-config` option.
 
Use this method when multiple options are to be specified i.e. both injection and modification. See example .json file
in the `tests` subdirectory .

Using this method, a dual configuration option may be chosen, allowing both injection and modification passes. However, 
only one modification type can be performed at a time for each available type, so attempting to
change two section headers, for example, is considered invalid input, whereas changing a program header
as well as a section header is valid.


## Tests and Example Code ##

The `build.sh` script will make test code in the `tests` subdir. Run with subcommand `help` to see 
available build options.' A README provides further description of each test source
TODO: Needs vigorous testing on other systems that can run different binary types (check on ARM aarch32 and aarch64, 
big endian x86 and x86-64, etc.)



## Current Working Status ##

    [x] Byte injection at end of a specified section (input as string)
    [ ] Byte injection at specified byte offset (input as hex)
    [ ] Byte injection replacing a section's bytes, rather than being added to the end of a section
    [x] Exec Header modification 
    [x] Section header modification
    [x] Program header modification
    [ ] Adding new segments & program header entry
    [ ] Adding new sections & section header entry
    [ ] Configuration file parsing
    [x] Helper utility for trimming byte slices for injection testing


## At some point may also add options for... #

    [ ] Modifying symbol entries and associated string tables
    [ ] Modifying rel/rela entries
    [ ] Output detailed ELF info organized by valid condifuration/modification options

