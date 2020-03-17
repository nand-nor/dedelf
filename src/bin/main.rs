extern crate ded_elf;

use std::process;

use ded_elf::{config, dedelf};

fn main() {
    let mut infile: String = "".to_string();
    let mut outfile: String = "".to_string();
    let mut options = config::DedElfOps::no_ops();
    match config::parse_args(&mut infile, &mut outfile, &mut options) {
        Ok(()) => match dedelf::run(infile, options, outfile) {
            Err(err) => {
                println!("\nDEDelf: Exiting due to error: {:?}", err);
                process::exit(1);
            }
            Ok(()) => {
                println!("\nDEDelf: successful byte edits written to file, exiting.");
                process::exit(0);
            }
        },
        Err(err) => {
            println!("\nDEDelf: Exiting, argument error: {:?}", err);
            ded_elf::display_useage();

            process::exit(1);
        }
    }
}
