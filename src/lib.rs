#![feature(in_band_lifetimes)]
#![feature(trivial_bounds)]
#![feature(arbitrary_enum_discriminant)]

extern crate byteorder;

pub mod config;
pub mod header;
pub mod parser;
pub mod section;
pub mod segment;
pub mod dedelf;
pub mod symbols;
pub mod relocations;



use std::env;
use std::process;


pub fn display_useage() {
    println!("\
\nUseage:   {}   [MODE]  [INFILE]  ...  [OUTPUT]  [OPTIONS] ... \n\
\nMODES: \n\
\t-i, --inject:\n\
\t\tinject mode requires minimum 2 file arguments (in this order):\n\
\t\t<file to modify> <file to inject> may be optionally followed \n\
\t\tby specifying output mode along with desired outfile name, \n\
\t\tfollowed by any combination of the 3 options listed below.\n\
\t-m, --modify: \n\
\t\tmodify mode requires the following specifications: the file \n\
\t\tto modify, followed by the header type to modify: \n\
\t\t\t`exec_header`\n\
\t\t\t`sec_header`\n\
\t\t\t`prog_header`\n\
\t\tTo modify the exec header, provide the value to change (using\n\
\t\tELF specification exec header struct fields) followed by \n\
\t\ta valid replacement type. For section and program header \n\
\t\tmodifications, provide a valid option (as found in section\n\
\t\theader and program header struct fields) followed by the \n\
\t\tsection name (for section headers with string table entries\n\
\t\tonly) or header index (for either section or program) within\n\
\t\tthe respective header table, and specify replacement value.\n\
\nOUTPUT:\n\
\t-o <output file>\n\t\tUse -o to specify a filename to write \n\
\t\tcontents to. If not specified, the default of `_inj` \n\
\t\tappended to the input file name will be used. \n\
\t\tOutput may be specified for either inject or modify mode. \n\
\nOPTIONS:\n\
\t-s <size in base 16>\n\
\t\tSet the size of the created injection \n\
\t\tsite to the specified size in bytes, rounding up to the\n\
\t\tclosest page size. If unset, default size is 0x1000\n\
\t-p <valid section name>\n\
\t\tSet the section that gets extended to the value as long as it \n\
\t\tis valid e.g. has an entry in the section header string table.\n\
\t\tif not set, the .text section is the default extended section\n\
\t-e <offset in base 16>\n\
\t\tChange the entry point of the executive header to the value\n\
\t\trelative to the byte offset of the injected bytes, \n\
\t\te.g., if a value of 0x10 is supplied, and the injection\n\
\t\t site starts at byte offset 0x1000, then the entry point \n\
\t\tin the executive header will be modified to be 0x1010\n\
\nExamples:\n\
\n{} -m path/to/target/file exec_header eh_entry 0x50250 \n\
\tThis command will modify the entry point in the executive header\n\
\tto byte offset 0x50250 and write the modified file to\n\
\tpath/to/target/file_inj\n\
\n{} -i path/to/target path/to/inj_bytes -e 0x500 -s 0x2000 -p .rodata\n\
\tThis command will inject the target file wiith the bytes contained\n\
\twithin injection_bytes, set the section to modify to `.rodata`,\n\
\tset the  size of the injection bytes to 0x2000, and will modify\n\
\tthe executive header to set the entry point to 0x500 \n\
\tFinally, the modified bytes are written to a new file to \n\
\tpath/to/target_inj\n",
             env::args().nth(0).unwrap(),
             env::args().nth(0).unwrap(),
             env::args().nth(0).unwrap()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
