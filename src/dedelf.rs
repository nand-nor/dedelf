//use crate::bytes;
use crate::parser;
use crate::parser::ElfParser;
use crate::config;
use crate::header;
use crate::header::*;

use crate::section::*;
use crate::segment::*;

use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom,};

use intervaltree;
use crate::config::ModOps;

pub struct Elf {
    parser: ElfParser,
    ops: config::DedElfOps,
}

pub trait DedElf {
    fn new(file: String, cfg: config::DedElfOps) -> Result<Self, std::io::Error> where Self: Sized;
    fn inject_or_modify(&mut self)-> Result<(), std::io::Error>;
    fn inject(&mut self,)-> Result<(), std::io::Error>;
    fn modify(&mut self,)-> Result<(), std::io::Error>;
    fn write(&self, file_ptr: &mut File) -> Result<(), std::io::Error>;
    fn write_exec_header(&self, file_ptr: &mut File) -> Result<(), std::io::Error>;
    fn write_header_tables(&self, file_ptr: &mut File) -> Result<(), std::io::Error>;
    fn write_segments(&self, file_ptr: &mut File) -> Result<(), std::io::Error>;
    fn write_sections(&self, file_ptr: &mut File) -> Result<(), std::io::Error>;
}

impl DedElf for Elf {
    fn new(file: String, cfg: config::DedElfOps) -> Result<Self, std::io::Error> where Self: Sized {
        Ok(Elf {
            parser:  parser::ElfParser::new(file)?,
            ops: cfg,
        })
    }

    //match statements no longer nested to allow for both injection and modification
    //in the same paoss -- only possible when using a config file
    fn inject_or_modify(&mut self)-> Result<(), std::io::Error> {

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

    fn inject(&mut self)-> Result<(), std::io::Error> {
        if let Some(inj) = &self.ops.injection {
            if let Some(section) = inj.get_extend() {
                let size = inj.get_size();
                // let section = ".text".to_string();//inj.get_extend();
                let entry = inj.get_entry();
                let mut tree_segs = vec![];

                for i in 0..self.parser.segments.len() {
                    let _left = self.parser.segments[i].offset();

                    let left: u64 = match _left {
                        PHTOffset::ThirtyTwo(left) => { left as u64 },
                        PHTOffset::SixtyFour(left) => { left },
                    };

                    let right = left + self.parser.segments[i].file_size() as u64;
                    tree_segs.push((left..right, i as u64));
                }

                let tree: intervaltree::IntervalTree<u64, u64> = tree_segs.iter().cloned().collect();

                let off = self.parser.get_section_offset_by_name(section.as_str());

                if off.is_none() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                   "Invalid Extend Section Entered"))
                }

                let offset = *off.unwrap();
                let mut sec_size = 0;

                for i in 0..self.parser.sections.len() {
                    //TODO resolve this
                    if self.parser.sections[i].name() == section {
                        sec_size = self.parser.sections[i].size() as usize;
                        break;
                    }
                }

                let point: Vec<u64> = tree
                    .query_point(offset as u64)
                    .map(|x| x.value)
                    .collect();

                let inj_file = self.ops.get_inj_file();
                if inj_file.is_none() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                   "No file provided, exiting"))
                }
                let inj_file = inj_file.unwrap();
                let mut fp = match File::open(inj_file.clone()) {
                    Err(why) => {
                        println!("Could not open target injection file: {}: {}",
                                 inj_file, why.description());
                        return Err(why)
                    },
                    Ok(fp) => fp,
                };


                let inj_bytes = parser::read_input(&mut fp)?;

                let _inj_site = self.parser.modify_segment(point[0] as usize,
                                                           offset as u64,
                                                           Some(sec_size),
                                                           size,
                                                           inj_bytes.to_vec())?;

                self.parser.update_segment_offsets(point[0], size as u64);

                self.parser.increase_sht_offset(size as u64)?;

                //TODO NEED TO MAKE AN INTERVAL TREE OF THE SECTONS AS WELL FOR THE BYTE OFFSET OPTION
                self.parser.update_secheader_offsets(section, size as u64);
                // let old_entry = self.parser.header.entry();

                if let Some(entry) = entry {
                    self.parser.header.update_exec_header("e_entry".to_string(),
                                                          entry as u64,
                                                          None)?;
                }
                return Ok(())
            } else if let Some(offset) = inj.get_offset() {



                return Ok(())
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "Invalid Config Options"))
            }
        }
        return Ok(())

    }

    fn modify(&mut self)-> Result<(), std::io::Error>{
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
                                        match header::match_osabi_as_str(replacement){
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
                        return Ok(())
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
                                                           hex value?"))
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
                                                           hex value?"))
                        }
                        val = check.unwrap();
                    }
                };
                self.parser.header.update_exec_header(field, val as u64, None)?;
                return Ok(())
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
                                                           hex value?"))
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
                    config::SegModOps::TYPE =>{
                        match_p_type_as_str(replacement)? as u64
                    },
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
                                                           hex value?"))
                        }
                        check.unwrap()
                    }
                };
                self.parser.update_seg_header(seg.seg_idx,field, val)?;
                } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "Invalid Config Options"))
            }
            return Ok(())
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "Invalid Config Options"))
        }
    }


    fn write(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        self.write_sections(file_ptr)?;
        self.write_segments(file_ptr)?;
        self.write_header_tables(file_ptr)?;
        self.write_exec_header(file_ptr)?;
        Ok(())
    }

    fn write_segments(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        for i in (0..self.parser.segments.len()).rev(){
            self.parser.segments[i].write_segment(file_ptr)?;
        }
        Ok(())
    }


    fn write_exec_header(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        self.parser.header.write_exec_header(file_ptr)?;
        Ok(())
    }

    fn write_header_tables(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
       // let sh_size= self.parser.header.sh_entry_size();
        //let ph_size = self.parser.header.ph_entry_size();

       let sht_offset =  match self.parser.header.sht_offset() {
            SHTOffset::ThirtyTwo(off) => {
                off as u64
            }
            SHTOffset::SixtyFour(off) => {
                off
            }
        };

        let pht_offset = match self.parser.header.pht_offset() {
            PHTOffset::ThirtyTwo(off) => {
                off as u64
            }
            PHTOffset::SixtyFour(off) => {
                off
            }
        };

        file_ptr.seek(SeekFrom::Start(sht_offset))?;

        for sec in &self.parser.sections {
            match &sec.SH {
                SecHeader::ThirtyTwo(secheader) => {
                    match self.parser.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            secheader.write_sec_header::<byteorder::LittleEndian>(file_ptr)?;
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            secheader.write_sec_header::<byteorder::BigEndian>(file_ptr)?;
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Cant write Elf"))
                    }
                }
                SecHeader::SixtyFour(secheader) => {
                    match self.parser.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            secheader.write_sec_header::<byteorder::LittleEndian>(file_ptr)?;
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            secheader.write_sec_header::<byteorder::BigEndian>(file_ptr)?;
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Cant write Elf"))
                    }
                }
            }
        }

        file_ptr.seek(SeekFrom::Start(pht_offset))?;

        for seg in &self.parser.segments {
            match &seg.PH {
                ProgHeader::ThirtyTwo(progheader) => {
                    match self.parser.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            progheader.write_prog_header::<byteorder::LittleEndian>(file_ptr)?;
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            progheader.write_prog_header::<byteorder::BigEndian>(file_ptr)?;
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Cant write Elf"))
                    }
                }
                ProgHeader::SixtyFour(progheader) => {
                    match self.parser.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            progheader.write_prog_header::<byteorder::LittleEndian>(file_ptr)?;
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            progheader.write_prog_header::<byteorder::BigEndian>(file_ptr)?;
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Cant write Elf"))
                    }
                }
            }
        }
        Ok(())
    }

    fn write_sections(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        for sec in &self.parser.sections {
            sec.write_section(file_ptr)?;
        }
        Ok(())
    }
}

/*
* run: perform the requested operations on the input elf.
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
            return Err(why)
        },
        Ok(fp) => fp,
    };
    return ded_elf.write(&mut fp);
}
