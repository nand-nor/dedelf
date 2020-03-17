# DEDelf: Deep elf EDitor #

### NOTE: DEDelf is a Work-In-Progress and is intended for research purposes only. See below for current working status ##

## Description ##

DEDelf is an ELF binary utility implemented in Rust, and intended to have utility similar to `elfedit` but with many more options. 
Use DEDelf to:

    1. Modify and Add ELF headers:
        - Modify any field in the executive header 
        - Modify any field in a specified section header
        - Modify any field in a specified program header
        - Add new sections / section headers
        - Add new segments / program headers
    2. Inject bytes within existing sections/segments
        - Inject additional bytes into to existing segments
        - Overwrite/Replace bytes within existing segments

## Command Line Useage ##

### Injection Mode ###

To specify injection mode, use `inject` as a positional argument, followed by `path/to/infile`, then using `-i` to 
specify the location to read injection bytes from. 
Optionally provide: 
1. `-p <name>`   : section name to place the bytes at the end of (default section is the `.text` section, assuming the infile has such a section. If not then the bytes will be ? (TODO))
2. `-b <offset>` : byte offset to inject bytes at (cannot be specified with `-p` option)
3. `-s <size>`   : page-aligned size (in hex) of injection bytes (default size is 0x1000)
4. `-e <entry>`  : new entry point (in hex) to modify the exec header with (default is no entry modification)
5. `--overwrite` : replace the bytes of the entire section, rather than appending injection bytes to the end of it. **Note: currently no support for when supplied with the `-b` option (will be ignored)**.
6. `-o <outfile>`: filename to write modified bytes to (default is to copy the infile name and append `_inj` to the string).

The bare minimum commands for injection mode are:
```
 dedelf inject <path/to/infile> -i <path/to/injection-bytes> 
```
With these provided command line arguments, the infile will be injected with the bytes of the file specified by the `-i`
option with the following default values: at the end of the `.text` section, of a total size of `0x1000` bytes, and written
to `path/to/infile_inj`. The entry point will not be modified. 

To replace the bytes in the `.text` section, use the`--overwrite` option as such:

```
 dedelf inject <path/to/infile> -i <path/to/injection-bytes> --overwrite
```
Size restrictions: In this case, the bytes to inject must be less than or equal to the number of bytes to 
overwrite. In the less-than case, the remaining portion will be overwritten with 0s. In
both the overwrite and append-to-end-of-section byte injection cases, the total size of the file containing injection bytes must not be larger than 0x1000 bytes. Use the `-s` 
option to specify a total byte size, bearing in mind that this value must be page aligned. 

### Modification Mode ###

To specify modification mode, use `modify` as a positional argument, followed by a file to modify, then use `-m` to 
specify a valid modification type, `-f` to specify a valid header field, `-r` to specify a valid replacement value, 
and, for section and program header modifications, specify which header by `-p` and providing either an index, in hex, 
for both program and section headers, or a valid section name (for just section header modifications). 

```
 dedelf modify <path/to/infile> -m <modification-type> [-p <index or name>] -f <header-field> -r <replacement> [-o <path/to/outfile>]
```

Valid modification types and valid fields are described as follows:

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
 dedelf modify path/to/infile -m exec_header -f EI_CLASS -r ELFCLASS32
 dedelf modify path/to/infile -m exec_header -f EI_DATA -r ELFDATA2MSB
 dedelf modify path/to/infile -m exec_header -f e_shoff -r 0x40100
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
dedelf modify path/to/infile -m sec_header -p .text -f sh_type -r SHT_RELA
dedelf modify path/to/infile -m sec_header -p 0xa -f sh_type -r SHT_RELA
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
dedelf modify path/to/infile -m prog_header -p 0x2 -f p_type -r PT_LOAD
```


For all subcommands and modes, to write the contents to a desired file name, append `-o path/to/outfile`.


`new_sec` : Not yet implemented. Add new section with corresponding entry in the section header table
Restricted to config file mode.

`new_seg` : Not yet implemented. Add new segment with corresponging entry in the program header table
Restricted to config file mode.

Optionally, a file to write modifications to may be provided as such:
 `-o <outfile>`: filename to write modified bytes to (default is to copy the infile name and append `_inj` to the string).


## Config file useage ##

**NOTE: This functionality does not yet exist in full! Eventually there will be a .json parser that can pull all of the configuration details and populate the required ops fields.** 

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
    [x] Byte injection at specified byte offset (input as hex)
    [x] Byte injection replacing a section's bytes, rather than being added to the end of a section
    [x] Exec Header modification 
    [x] Section header modification
    [x] Program header modification
    [x] Helper utility for trimming byte slices for injection testing
    [ ] Adding new segments & program header entry (see issue #3)
    [ ] Adding new sections & section header entry (see issue #4)
    [ ] Configuration file parsing (see issue #5)

## Future Work / Possible Enhancements #

    [ ] Moodifying symbol entries and associated string tables
    [ ] Modifying rel/rela entries
    [ ] Search for specific byte patterns of a certain size & inject / overwrite that point
    [ ] Output detailed ELF info organized by valid condifuration/modification options
    [ ] Option to patch additional bytes at end of injection to jump back to original entry point (as of right now the user is expected to encode this functionality within injected byte slice)
