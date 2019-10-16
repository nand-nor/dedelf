extern crate ded_elf;

use std::env;
use std::process;

use ded_elf::{config, dedelf};

fn main() {
    //only allow between 4 and 11 args
    if env::args().len() < 4 || env::args().len() > 12 {
        ded_elf::display_useage();
        process::exit(1);
    }

    let mut infile: String = "".to_string();
    let mut outfile: String = "".to_string();
    let mut options = config::DedElfOps::no_ops();
    config::parse_args(&mut infile, &mut outfile, &mut options);

    match dedelf::run(infile, options, outfile) {
        Err(err) => {
            println!("\nDEDelf: Exiting due to error: {:?}", err);
            process::exit(1);
        }
        Ok(()) => {
            println!("\nDEDelf: successful byte edits written to file, exiting.");
            process::exit(0);
        }
    }
}
