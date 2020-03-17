//use crate::bytes;
use crate::parser;
use crate::parser::ElfParser;
use crate::config;
use crate::header;
use crate::header::*;

use crate::section::*;
use crate::segment::*;

use std::fs::File;
use std::error::Error;

use intervaltree;

pub struct Elf {
    parser: ElfParser,
    ops: config::DedElfOps,
}

pub trait DedElf {
    fn new(file: String, cfg: config::DedElfOps) -> Result<Self, std::io::Error> where Self: Sized;
    fn inject_or_modify(&mut self) -> Result<(), std::io::Error>;
    fn inject(&mut self) -> Result<(), std::io::Error>;
    fn modify(&mut self) -> Result<(), std::io::Error>;
    fn write(&self, file_ptr: &mut File) -> Result<(), std::io::Error>;
}

impl DedElf for Elf {
    fn new(file: String, cfg: config::DedElfOps) -> Result<Self, std::io::Error> where Self: Sized {
        Ok(Elf {
            parser: parser::ElfParser::new(file)?,
            ops: cfg,
        })
    }

    //match statements no longer nested to allow for both injection and modification
    //in the same pass -- only possible when using a config file
    fn inject_or_modify(&mut self) -> Result<(), std::io::Error> {
        match &self.ops.injection {
            Some(_inj) => {
                println!("\nDEDelf: running injection mode...");
                self.inject()?;
            }
            None => {
                // println!("No injection options specified");
            }
        };
        match &self.ops.modify {
            Some(_modify) => {
                println!("\nDEDelf: running modification mode...");
                self.modify()?;
            }
            None => {
                // println!("No modification options specified");
            }
        };

        Ok(())
    }

    fn inject(&mut self) -> Result<(), std::io::Error> {
        if let Some(inj) = &self.ops.injection {
            let mut offset: u64 = 0;
            let mut sec_size = None;
            let mut tree_segs = vec![];
            let replace = inj.get_replace();
            /*generate interval tree using segment bytes as bounds*/
            for i in 0..self.parser.segments.len() {
                let _left = self.parser.segments[i].offset();

                let left: u64 = match _left {
                    PHTOffset::ThirtyTwo(left) => { left as u64 }
                    PHTOffset::SixtyFour(left) => { left }
                };

                let right = left + self.parser.segments[i].file_size() as u64;
                tree_segs.push((left..right, i as u64));
            }

            let seg_tree: intervaltree::IntervalTree<u64, u64> = tree_segs.iter().cloned().collect();
            let size = inj.get_size();
            let entry = inj.get_entry();
            let mut section: String = " ".to_string();

            if let Some(_section) = inj.get_extend() {
                let off = self.parser.get_section_offset_by_name(_section.as_str());

                if off.is_none() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                   "Invalid Extend Section Entered"));
                }

                offset = *off.unwrap() as u64;
                //let mut sec_size = 0;
                section = _section;
                for i in 0..self.parser.sections.len() {
                    //TODO resolve this
                    if self.parser.sections[i].name() == section {
                        sec_size = Some(self.parser.sections[i].size() as usize);
                        break;
                    }
                }
            } else if let Some(_offset) = inj.get_offset() {
                offset = _offset;
                let mut tree_secs = vec![];

                /*generate interval tree using section bytes as bounds*/
                for i in 0..self.parser.sections.len() {
                    let left = self.parser.sections[i].offset();
                    let right = left + self.parser.sections[i].size() as u64;
                    tree_secs.push((left..right, i as u64));
                }

                let sec_tree: intervaltree::IntervalTree<u64, u64> =
                    tree_secs.iter().cloned().collect();

                /*return the section index that contains the specified byte offset*/
                let sec_point: Vec<u64> = sec_tree
                    .query_point(offset as u64)
                    .map(|x| x.value)
                    .collect();

                section = self.parser.sections[sec_point[0] as usize].name();
            }

            let point: Vec<u64> = seg_tree
                .query_point(offset as u64)
                .map(|x| x.value)
                .collect();

            let inj_file = self.ops.get_inj_file();
            if inj_file.is_none() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "No file provided, exiting"));
            }

            let inj_file = inj_file.unwrap();
            let mut fp = match File::open(inj_file.clone()) {
                Err(why) => {
                    println!("Could not open target injection file: {}: {}",
                             inj_file, why.description());
                    return Err(why);
                }
                Ok(fp) => fp,
            };


            let inj_bytes = parser::read_input(&mut fp)?;
            let _inj_site = self.parser.modify_segment(point[0] as usize,
                                                       offset as u64,
                                                       sec_size, replace,
                                                       size,
                                                       inj_bytes.to_vec())?;

            self.parser.update_segment_offsets(point[0], size as u64);
            self.parser.increase_sht_offset(size as u64)?;
            self.parser.update_secheader_offsets(section, size as u64);
            if let Some(entry) = entry {
                self.parser.header.update_exec_header("e_entry".to_string(),
                                                      entry as u64,
                                                      None)?;
            }
            return Ok(());
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "Invalid Config Options"));
        }
    }

    fn modify(&mut self) -> Result<(), std::io::Error> {
        if let Some(modify) = &self.ops.modify {
            if let Some(exec) = &modify.exec {
                let replacement = exec.replacement.clone();
                let field = config::get_exec_field(exec.op_mode);
                let val: u64;// = 0;
                match exec.op_mode {
                    config::ExecModOps::IDENT => {
                        let (val, offset) =
                            match header::match_class_as_str(replacement.clone()) {
                                Err(_) => {
                                    match header::match_data_as_str(replacement.clone()) {
                                        Err(_) => {
                                            match header::match_osabi_as_str(replacement) {
                                                Err(err) => return Err(err),
                                                Ok(val) => (val, header::EXEC::_EI_OSABI)
                                            }
                                        }
                                        Ok(val) => (val, header::EXEC::_EI_DATA)
                                    }
                                }
                                Ok(val) => (val, header::EXEC::_EI_CLASS)
                            };
                        self.parser.header.update_exec_header("e_ident".to_string(),
                                                              val as u64,
                                                              Some(offset))?;
                        return Ok(());
                    }
                    config::ExecModOps::TYPE => {
                        val = header::match_type_as_str(replacement)? as u64;
                    }
                    config::ExecModOps::MACH => {
                        val = header::match_mach_as_str(replacement)? as u64;
                    }
                    config::ExecModOps::VERSION => {
                        val = header::match_version_as_str(replacement)? as u64;
                    }
                    config::ExecModOps::ENTRY |
                    config::ExecModOps::PHOFF |
                    config::ExecModOps::SHOFF => {
                        let trimmed = replacement.trim_start_matches("0x");

                        let check = u64::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                           "Invalid Config Options: \
                                                           entry or offset value is invalid \
                                                           did you provide a valid \
                                                           hex value?"));
                        }
                        val = check.unwrap();
                    }
                    config::ExecModOps::FLAGS |
                    config::ExecModOps::EHSIZE |
                    config::ExecModOps::PHENTSIZE |
                    config::ExecModOps::PHNUM |
                    config::ExecModOps::SHENTSIZE |
                    config::ExecModOps::SHNUM |
                    config::ExecModOps::SHSTRNDX

                    => {
                        let trimmed = replacement.trim_start_matches("0x");
                        let check = u64::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                           "Invalid Config Options: \
                                                           provided value is invalid \
                                                           did you provide a valid \
                                                           hex value?"));
                        }
                        val = check.unwrap();
                    }
                };
                self.parser.header.update_exec_header(field, val as u64, None)?;
                return Ok(());
            } else if let Some(sec) = &modify.sec {
                let field = config::get_sec_field(sec.op_mode);
                let replacement = sec.replacement.clone();
                let val: u64 = match &sec.op_mode {
                    config::SecModOps::TYPE => {
                        match_sh_type_as_str(replacement)? as u64
                    }
                    config::SecModOps::FLAGS => {
                        match_sh_flag_as_str(replacement)?
                    }
                    config::SecModOps::NAME |
                    config::SecModOps::ADDR |
                    config::SecModOps::OFFSET |
                    config::SecModOps::SIZE |
                    config::SecModOps::LINK |
                    config::SecModOps::INFO |
                    config::SecModOps::ADDRALIGN |
                    config::SecModOps::ENTSIZE
                    => {
                        let trimmed = replacement.trim_start_matches("0x");
                        let check = u64::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                           "Invalid Config Options: \
                                                           provided sec header replacement value \
                                                           is invalid \
                                                           did you provide a valid \
                                                           hex value?"));
                        }
                        check.unwrap()
                    }
                };
                self.parser.update_sec_header(sec.sec_name.clone(),
                                              sec.sec_idx,
                                              field,
                                              val)?;
            } else if let Some(seg) = &modify.seg {
                let field = config::get_seg_field(seg.op_mode);
                let replacement = seg.replacement.clone();
                let val: u64 = match &seg.op_mode {
                    config::SegModOps::TYPE => {
                        match_p_type_as_str(replacement)? as u64
                    }
                    config::SegModOps::FLAGS => {
                        match_p_flag_as_str(replacement)? as u64
                    }
                    config::SegModOps::OFFSET |
                    config::SegModOps::VADDR |
                    config::SegModOps::PADDR |
                    config::SegModOps::FILESZ |
                    config::SegModOps::MEMSZ |
                    config::SegModOps::ALIGN => {
                        let trimmed = replacement.trim_start_matches("0x");
                        let check = u64::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                           "Invalid Config Options: \
                                                           provided sec header replacement value \
                                                           is invalid \
                                                           did you provide a valid \
                                                           hex value?"));
                        }
                        check.unwrap()
                    }
                };
                self.parser.update_seg_header(seg.seg_idx, field, val)?;
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "Invalid Config Options"));
            }
            return Ok(());
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "Invalid Config Options"));
        }
    }


    fn write(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        self.parser.write_sections(file_ptr)?;
        self.parser.write_segments(file_ptr)?;
        self.parser.write_header_tables(file_ptr)?;
        self.parser.write_exec_header(file_ptr)?;
        Ok(())
    }

}

/*
* fn run: perform the requested operations on the input file (must contain a valid ELF).
* 1. Create the generic Elf object that implements the DedElf trait
* 2. Perform the requested injection and/or modifications
* 3. Write the modified bytes to the provided outfile
*/
pub fn run(file: String, ops: config::DedElfOps, outfile: String) -> Result<(), std::io::Error> {
    let mut ded_elf: Elf = DedElf::new(file, ops)?;
    ded_elf.inject_or_modify()?;
    let mut dir_path = std::env::current_dir().unwrap();
    dir_path.push(outfile.as_str());

    let mut fp = match File::create(dir_path.as_path()) {
        Err(why) => {
            println!("Could not open target output file: {}: {}",
                     dir_path.as_path().display(), why.description());
            return Err(why);
        }
        Ok(fp) => fp,
    };
    return ded_elf.write(&mut fp);
}
