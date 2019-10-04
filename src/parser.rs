
use std::error::Error;

use std::fs::File;
use std::path::PathBuf; 
use std::io::*;

use std::io::{Read, Seek, SeekFrom};


pub struct ElfParser<R>{
    fp: R,
     
}
/*
impl std::io::Read for ElfParser {
    fn read_to_end(&mut self, input: &mut Vec<u8>) {
      //  self.fp.    

    }
}*/
/*
impl std::io::Seek for ElfParser{


}

impl std::io::SeekFrom for ElfParser{


}
*/
impl <R>ElfParser<R> where R: Read + Seek{
        



}
