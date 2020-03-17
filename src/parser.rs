use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashMap;
use crate::header::*;
use crate::symbols::*;

use crate::section::{Strtab, Section, SecHeader, SecHeader32, SecHeader64, SH_Type};
use crate::segment::{Segment, ProgHeader, ProgHeader32, ProgHeader64};

use byteorder::*;

pub struct ElfParser{
   pub header: ExecutiveHeader,

   pub segments: Vec<Segment>,
   pub sections: Vec<Section>,


    //TODO refine these data structures for fast parsing & manipulation of symbols/strings/rel/rela
    // Right now these are mainly placeholders
    pub sechdrstr: HashMap<String, usize>,
    pub symbols: HashMap<String, usize>,
    pub dynstr: HashMap<String, usize>,
    pub relsym: HashMap<String, usize>,
    pub relasym: HashMap<String, usize>,
    pub dynsym: HashMap<String, usize>,

    //TODO use in support of more options for modifying/manipulating strings and symbols
    pub string_tables: Vec<Strtab>,
    pub sym_tables: Vec<Symtable>,
    pub dynsym_tables: Vec<DynSymtable>,

    //pub shtstr_tab: Box<Vec<u8>>, Dont need-- captured in string_tables vector
    //pub dyn_str: Box<Vec<u8>>, Dont need-- captured in string_tables vector


    //TODO create additional structures to map in O(1) time offsets to names, indexes to offsets,
    // and others as needed. Lots of possible optimizations, like perhaps storing
    // segment and section interval trees within this object rather than creating for injection?
    pub sec_offsets: HashMap<String, usize>,
    pub seg_offsets: HashMap<usize, usize>,

    //TODO use for testing
    pub size: usize,
}



pub fn read_input<R>(file_ptr: &mut R) -> Result<Vec<u8>,std::io::Error>
    where
        R: Read + Seek,
{
    let mut buf = vec![];
    file_ptr.read_to_end(&mut buf)?;
    Ok(buf)
}

/* TODO make this take just a byte slice! Not the whole byte string
* Given the string table as a utf8 string and a string table (byte offset) index, return
* the string at the offset as an ascii-readable, trimmed value
*/
pub fn get_strtable_entry(byte_string: String, val: u32) -> String {
    let len = byte_string.len();
    let pos = byte_string[val as usize..len].chars()
        .position(|c| c == '\u{0}')
        .unwrap();
    let search_str =
        &byte_string[val as usize..(val as usize + pos) as usize];
    search_str.to_string()
}

impl ElfParser {
    pub fn new(infile: String) -> Result<ElfParser, std::io::Error> {
        let mut file_ptr = match File::open(infile) {
            Err(why) => {
                println!("Could not open input target input: {}", why.description());
                return Err(why)
            },
            Ok(fp) => fp,
        };

        let header: ExecutiveHeader = ExecutiveHeader::new(&mut file_ptr)?;
        let mut parser = ElfParser {
            header: header,
            size: 0,
            segments:  Vec::new(),
            sections:  Vec::new(),
            string_tables: Vec::new(),
            sym_tables:  Vec::new(),
            dynsym_tables:  Vec::new(),
            sechdrstr: HashMap::new(),
            symbols: HashMap::new(),
            dynstr: HashMap::new(),
            relsym: HashMap::new(),
            relasym: HashMap::new(),
            dynsym: HashMap::new(),
            sec_offsets: HashMap::new(),
            seg_offsets: HashMap::new()
        };

        parser.parse_segments(&mut file_ptr)?;
        parser.parse_sections(&mut file_ptr)?;
        Ok(parser)
    }

    fn parse_segments<R>( &mut self, file_ptr: &mut R,) -> Result<(), std::io::Error>
        where R: Read + Seek, {
        let pht_offset = match self.header.pht_offset() {
            PHTOffset::ThirtyTwo(offset) => offset as u64,
            PHTOffset::SixtyFour(offset) => offset,
        };

        let mut pht_bytes_t: Vec<u8>;

        let ph_num = self.header.ph_entry_num();
        let ph_size = self.header.ph_entry_size();
        for i in 0..ph_num {
            let _ph_offset = pht_offset as u64 + (i as u64 * ph_size as u64);

            file_ptr.seek(SeekFrom::Start(_ph_offset.into()))?;

            let size = match self.header.class {
                EXEC::EI_CLASS::ELFCLASS32 => {
                    match self.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let header: ProgHeader32 =
                                ProgHeader32::parse_prog32_header::<R, LittleEndian>( file_ptr)?;

                            file_ptr.seek(SeekFrom::Start(header.offset().into()))?;
                            let size = header.mem_size();

                            pht_bytes_t = vec![0; size as usize];
                            file_ptr.read_exact(&mut pht_bytes_t)?;

                            self.segments.push(Segment {
                                PH: ProgHeader::ThirtyTwo(header),
                                raw_bytes: pht_bytes_t.to_vec(),
                            });
                            size as usize
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let header: ProgHeader32
                                = ProgHeader32::parse_prog32_header::<R, BigEndian>( file_ptr)?;

                            file_ptr.seek(SeekFrom::Start(header.offset().into()))?;
                            let size = header.mem_size();

                            pht_bytes_t = vec![0; size as usize];
                            file_ptr.read_exact(&mut pht_bytes_t)?;

                            self.segments.push(Segment {
                                PH: ProgHeader::ThirtyTwo(header),
                                raw_bytes: pht_bytes_t.to_vec(),
                            });
                            size as usize
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                },
                EXEC::EI_CLASS::ELFCLASS64 => {
                    match self.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let header: ProgHeader64 =
                                ProgHeader64::parse_prog64_header::<R, LittleEndian>(file_ptr)?;
                            file_ptr.seek(SeekFrom::Start(header.offset()))?;

                            let size = header.mem_size();

                            pht_bytes_t = vec![0; size as usize];
                            file_ptr.read_exact(&mut pht_bytes_t)?;

                            self.segments.push(Segment {
                                PH: ProgHeader::SixtyFour(header),
                                raw_bytes: pht_bytes_t.to_vec(),
                            });
                            size as usize
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let header: ProgHeader64
                                = ProgHeader64::parse_prog64_header::<R, BigEndian>(file_ptr)?;
                            file_ptr.seek(SeekFrom::Start(header.offset()))?;

                            let size = header.mem_size();

                            pht_bytes_t = vec![0; size as usize];
                            file_ptr.read_exact(&mut pht_bytes_t)?;

                            self.segments.push(Segment {
                                PH: ProgHeader::SixtyFour(header),
                                raw_bytes: pht_bytes_t.to_vec(),
                            });
                            size as usize
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                }
                _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                    "Elf not supported"))
            };
            self.size += size;
        }
        Ok(())
    }

    //TODO add support for stripped ELFs-- right now just exit if no section header names
    // are found (via names_flag check)
    fn parse_sections<R>(&mut self, file_ptr: &mut R,) -> Result<(), std::io::Error>
        where R: Read + Seek, {

        let mut byte_string: String = " ".to_string();
        let mut sht_bytes_t: Vec<u8>;

        let mut names_flag = false;

        let sht_offset = match self.header.sht_offset() {
            SHTOffset::ThirtyTwo(offset) => offset as u64,
            SHTOffset::SixtyFour(offset) => offset,
        };

        let sh_num = self.header.sh_entry_num();
        let sh_size = self.header.sh_entry_size();
        for i in 0..sh_num {
            let name = " - ";
            let _sh_offset = sht_offset as u64 + (i as u64 * sh_size as u64);

            file_ptr.seek(SeekFrom::Start(_sh_offset.into()))?;

            match self.header.class {
                EXEC::EI_CLASS::ELFCLASS32 => {
                    match self.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let header: SecHeader32
                                = SecHeader32::parse_sec32_header::<R, LittleEndian>( file_ptr)?;
                            file_ptr.seek(SeekFrom::Start(header.offset().into()))?;
                            sht_bytes_t = vec![0; header.size() as usize];
                            file_ptr.read_exact(&mut sht_bytes_t)?;

                            self.sections.push(Section {
                                SH: SecHeader::ThirtyTwo(header),
                                raw_bytes: sht_bytes_t.to_vec(),
                                name: name.to_string(),
                            });
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let header: SecHeader32
                                = SecHeader32::parse_sec32_header::<R, BigEndian>( file_ptr)?;

                            file_ptr.seek(SeekFrom::Start(header.offset().into()))?;
                            sht_bytes_t = vec![0; header.size() as usize];
                            file_ptr.read_exact(&mut sht_bytes_t)?;
                            self.sections.push(Section {
                                SH: SecHeader::ThirtyTwo(header),
                                raw_bytes: sht_bytes_t.to_vec(),
                                name: name.to_string(),
                            });
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                },
                EXEC::EI_CLASS::ELFCLASS64 => {
                    match self.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let header: SecHeader64
                                = SecHeader64::parse_sec64_header::<R, LittleEndian>(file_ptr)?;
                            file_ptr.seek(SeekFrom::Start(header.offset().into()))?;
                            sht_bytes_t = vec![0; header.size() as usize];
                            file_ptr.read_exact(&mut sht_bytes_t)?;

                            self.sections.push(Section {
                                SH: SecHeader::SixtyFour(header),
                                raw_bytes: sht_bytes_t.to_vec(),
                                name: name.to_string(),
                            });
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let header: SecHeader64
                                = SecHeader64::parse_sec64_header::<R, BigEndian>(file_ptr)?;

                            file_ptr.seek(SeekFrom::Start(header.offset().into()))?;
                            sht_bytes_t = vec![0; header.size() as usize];
                            file_ptr.read_exact(&mut sht_bytes_t)?;

                            self.sections.push(Section {
                                SH: SecHeader::SixtyFour(header),
                                raw_bytes: sht_bytes_t.to_vec(),
                                name: name.to_string(),
                            });
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                }
                _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                    "Elf not supported"))
            };

            /* Parse the section header string table into a utf8 string, for later name assignment*/
            if i == self.header.shstrndx() {
                let shstrtab_str = String::from_utf8(sht_bytes_t.clone());
                if shstrtab_str.is_err() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                   "Error: no section header string names"))
                }
                byte_string = shstrtab_str.unwrap();
                names_flag = true;
            }
        }

        if !names_flag {
            //TODO add support for searching by index rather than name, in case ELF has been
            // stripped! For now just exit as it will potentially mess up down-stream functionality if
            // no stirng names ar eprovided
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "No section header string names available, exiting with error"))
        }

        for itr in &mut self.sections {
            let name_idx = itr.name_idx();
            let name: String = get_strtable_entry(byte_string.clone(), name_idx);
            //println!("Name! is {:?}", name);
            let offset = itr.offset();

            self.sec_offsets.insert(name.clone(), offset as usize);

            itr.set_name(name.clone());
            let index = itr.link_idx() as usize;

            let sh_type = itr.shtype_as_enum()?;
            match sh_type {
                SH_Type::SHT_SYMTAB => {
                    file_ptr.seek(SeekFrom::Start(itr.offset().into()))?;

                    let symtab = Symtable::parse_sym_table::<R>( file_ptr,
                                                                 itr.size(),
                                                                 index as u32,
                                                                 name.clone(),
                                                                 self.header.data,
                                                                 self.header.class)?;
                    //println!("The symtab! {:?}", symtab);
                    self.sym_tables.push(symtab);
                    self.symbols.insert(name, index);
                }
                SH_Type::SHT_DYNSYM => {
                   // println!("Inserted section type DynSym with name {:?} and index {:?}", name.clone(), index);
                    file_ptr.seek(SeekFrom::Start(itr.offset().into()))?;


                    let dynsymtab = DynSymtable::parse_dynsym_table::<R>(file_ptr,
                                                                         itr.size(),
                                                                         index as u32,
                                                                         name.clone(),
                                                                         self.header.data,
                                                                         self.header.class)?;
                  //  println!("The dynsymtab! {:?}", dynsymtab);


                    self.dynsym_tables.push(dynsymtab);
                    self.dynsym.insert(name, index);
                }
                SH_Type::SHT_STRTAB => {
               //     println!("Creating string table with name {:?} and index {:?}", name.clone(), index);
                    let bytes = itr.raw_bytes();
                    let strtab = Strtab::parse_str_table(name.clone(), bytes);
                    self.string_tables.push(strtab);
                }
                SH_Type::SHT_RELA => {
                    self.relasym.insert(name, index);
                    //todo PARSE RELA SYMBOL DATA!
                }
                SH_Type::SHT_REL => {
                    self.relsym.insert(name, index);
                    //todo PARSE REL SYMBOL DATA!
                }
                SH_Type::SHT_DYNAMIC => {
                    self.dynsym.insert(name, index);
                }
                SH_Type::SHT_HASH => {
                    //TODO need to determine what kind of functionality to support for such sections
                  //  self.symbols.insert(name, index);
                }
                SH_Type::SHT_GNU_HASH => {
                    //TODO need support for this section
                  //  self.symbols.insert(name, index);
                }
                _ => {}
            }
        }

        //TODO add support for more manipulation/modification of symbols
        for symtab in &self.sym_tables {

            let symbytes = self.sections[symtab.section_idx as usize].raw_bytes().clone();
            let _ent_size = self.sections[symtab.section_idx as usize].entsize();
            let syms = String::from_utf8(symbytes.clone());
            if syms.is_err() {
                //println!("Error parsing symbols");
                break;
            }
            let syms = syms.unwrap();
            let len = syms.len();

            let mut output_vec: Vec<String> = Vec::new();
            for val in symbytes.clone() {
                let pos = syms[val as usize..len]
                    .chars().position(|c| c == '\u{0}')
                    .unwrap();

                let search_str =
                    &syms[val as usize..(val as usize + pos) as usize];
                //  index += 1;
                output_vec.push(search_str.to_string());
            }

            for i in 0..symtab.entries.len() {
                let (_sht_idx, tab_idx, _sym_size) = match &symtab.entries[i] {
                    Symbol::ThirtyTwo(sym) => (sym.st_shndx as u64, sym.st_name as u64, sym.st_size as u64),
                    Symbol::SixtyFour(sym) => (sym.st_shndx as u64, sym.st_name as u64, sym.st_size as u64)
                };

                if tab_idx == 0 {
                   // println!("Symbol has no name or no known size");

                    continue
                } else if tab_idx as usize >= symbytes.len() {
                    continue
                } else {

                    let symbol_name = String::from_utf8(symbytes[tab_idx as usize..].to_vec());

                    if symbol_name.is_err() {
                        //println!("Error parsing symbol");
                        break;
                    }

                    //TODO eventually will support
                    let symbol_name = symbol_name.unwrap();
                    let _name = symbol_name.split("\u{0}").collect::<Vec<_>>()[0];

                    //println!("Symbol name is {:?} table index is {:?}", name, tab_idx);

                }
            }
        }

        //TODO add support for manipulating/modifying dynamic symbols
        for symtab in &self.dynsym_tables {

            let symbytes = self.sections[symtab.section_idx as usize].raw_bytes().clone();
            let _ent_size = self.sections[symtab.section_idx as usize].entsize();
            let syms = String::from_utf8(symbytes.clone());//.unwrap();
            if syms.is_err() {
               // println!("Error parsing symbols");
                break;
            }
            let _syms = syms.unwrap();


            for i in 0..symtab.entries.len() {
                let (_sht_idx, tab_idx, _sym_size) = match &symtab.entries[i] {
                    DynSymbol::ThirtyTwo(sym) => (sym.st_shndx as u64, sym.st_name as u64, sym.st_size as u64),
                    DynSymbol::SixtyFour(sym) => (sym.st_shndx as u64, sym.st_name as u64, sym.st_size as u64)
                };
                 if tab_idx == 0 {
      //              println!("Symbol has no name or no known size");

                    continue
                } else if tab_idx as usize >= symbytes.len() {
                    continue
                } else {
                    let symbol_name = String::from_utf8(symbytes[tab_idx as usize..].to_vec());

                    if symbol_name.is_err() {
                    //    println!("Error parsing symbol");
                        break;
                    }

                     //TODO store these in hashmap for O(1) symbol lookup!
                    let symbol_name = symbol_name.unwrap();
                    let _name = symbol_name.split("\u{0}").collect::<Vec<_>>()[0];

                    //println!("Symbol name is {:?} table index is {:?}", name, tab_idx);
                }
            }
        }
        Ok(())
    }

        pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_section_offset_by_name(&self, section: &'a str) -> Option<&usize> {
        self.sec_offsets.get(section)
    }


    pub fn update_secheader_offsets(&mut self, after_sec: String, by_size: u64, ){
        let mut sec_flag = false;
        for sec in &mut self.sections {
            if sec.name() == after_sec {
                sec_flag = true;
                let old_size = sec.size();
                sec.set_size(old_size + by_size);

                continue
            }
            if sec_flag {
                sec.increase_offset(by_size);
            }
        }
    }

    //TODO need to test cases for increasing vaddr-- if offset and vaddr are no longer congruent,
    // I have increased the vaddr by the same increase as the offset. Using the example of
    // injecting 1 page (0x1000 bytes) into the previous segment, assuming it is a LOAD
    // segment, and the following to-be-changed segment is also LOAD, then I think it makes
    // sense that the VADDR increases to accommodate that extra page of bytes, but is it
    // possible that changing the alignment value makes any sense?
    pub fn update_segment_offsets(&mut self, seg_idx: u64, by_size: u64){
        for i in (seg_idx + 1)..self.segments.len() as u64 {
            self.segments[i as usize].increase_offset(by_size);

            let vaddr = self.segments[i as usize].vaddr();
            let align =  self.segments[i as usize].align();
            let offset: u64 = match self.segments[i as usize].offset(){
                PHTOffset::ThirtyTwo(offset)=>{offset as u64},
                PHTOffset::SixtyFour(offset)=>{offset},
            };


            if offset % align != vaddr % align && align != 1 && align != 0 {
                self.segments[i as usize].increase_vaddr(by_size);
            }
        }
    }

    /* Change the sht offset completely */
    pub fn update_sht_offset(&mut self, new_offset: u64)-> Result<(), std::io::Error>{
        self.header.update_exec_header("e_shoff".to_string(), new_offset, None)?;
        Ok(())
    }

    /* Increase sht offset by a fixed amount */
    pub fn increase_sht_offset(&mut self, by_size: u64) -> Result<(), std::io::Error>{
        let old_sht_offset = match self.header.sht_offset() {
            SHTOffset::ThirtyTwo(offset)=>{
                offset as u64
            }
            SHTOffset::SixtyFour(offset)=>{
                offset
            }
        };

        self.header.update_exec_header("e_shoff".to_string(),
                                       old_sht_offset + by_size,
                                       None)?;
        Ok(())
    }



/*
    pub fn change_exec_type(&mut self, edit_type: String,
                            replace: String,)-> Result<(), std::io::Error>{
        let etype = crate::header::match_type_as_str(replace)?;
        self.header.update_exec_header("e_type".to_string(), etype as u64,None)?;
        Ok(())
    }


    pub fn change_exec_data(&mut self, edit_type: String,
                            replace: String,)-> Result<(), std::io::Error>{
        let edata = crate::header::match_data_as_str(replace)?;
        self.header.update_exec_header("e_ident".to_string(),
                                       edata as u64,
                                       Some(EXEC::_EI_DATA as usize))?;
        Ok(())
    }


    pub fn change_exec_class(&mut self, edit_type: String,
                             replace: String) -> Result<(), std::io::Error>{
        let eclass = crate::header::match_class_as_str(replace)?;
        self.header.update_exec_header("e_ident".to_string(),
                                       eclass as u64,
                                       Some(EXEC::_EI_CLASS as usize))?;
        Ok(())
    }
*/
    pub fn modify_segment(&mut self, seg_idx: usize,
                          file_offset: u64, sec_size: Option<usize>, replace: bool,
                          inj_size: usize, bytes: Vec<u8>) -> Result<usize, std::io::Error>{

        let _off = self.segments[seg_idx].offset();
        let offset: u64 = match _off{
            PHTOffset::ThirtyTwo(offset)=>{offset as u64},
            PHTOffset::SixtyFour(offset)=>{offset},
        };

        let b_offset = (file_offset).checked_sub(offset);
        if b_offset.is_none(){
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "Invalid byte offset provided"))
        }


        if inj_size < bytes.len(){
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "Invalid injection size provided"))
        }


        let old_bytes = self.segments[seg_idx].raw_bytes.clone();


        let byte_offset: usize;

        if let Some(sec_size) = sec_size {
            if replace {
                byte_offset = b_offset.unwrap() as usize;
                if bytes.len() > sec_size {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                   "Invalid injection size provided with overwrite option"))
                }
            } else {
                byte_offset = b_offset.unwrap() as usize + sec_size;
            }
        } else {
            byte_offset = b_offset.unwrap() as usize;
        }


        let preserve_first = &old_bytes[0..byte_offset];
        let mut preserve_last = old_bytes[byte_offset..].to_vec();

        let mut increase_size: u64 = bytes.len() as u64;

        if let Some(sec_size) = sec_size {
            if replace {
                //if injection bytes are less than the section size, pad with zeros
                if bytes.len() < sec_size {
                    increase_size = sec_size as u64 - bytes.len() as u64;
                    let mut zeros = vec![0u8;increase_size as usize];
                    zeros.extend(&preserve_last[bytes.len() +1..]);
                    preserve_last = zeros;
                }
            }

        }

        let mut new_bytes = [&preserve_first[..], &bytes[..]].concat();
        new_bytes.extend(preserve_last.as_slice());

        self.segments[seg_idx].set_bytes(new_bytes.clone());

        self.segments[seg_idx].increase_size(increase_size);

        if let Some(sec_size) = sec_size {
            return  Ok(file_offset as usize + sec_size)
        } else {
            return Ok(file_offset as usize)
        }
    }

    pub fn update_sec_header(&mut self,name: Option<String>, index: Option<usize>,
                             field: String, val: u64)-> Result<(),std::io::Error> {
        if let Some(name) = name {
            let mut name_flag = false;
            for sec in &mut self.sections {
                if sec.name() == name {
                    name_flag = true;
                    sec.update_sec_header(field, val)?;
                    break;
                }
            }
            if !name_flag{
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "Invalid section name in mod options provided"))
            }
        } else if let Some(index) = index {
            if index < self.sections.len() {
                self.sections[index].update_sec_header(field, val)?;
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "Invalid sec header index provided"))
            }

        } else {
            println!("No valid sec modify options provided");

            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "Invalid sec header mod options provided"))

        }
        Ok(())
    }


    pub fn update_seg_header(&mut self, index: usize, field: String,
                             val: u64)-> Result<(),std::io::Error> {
        if index < self.segments.len() {
            self.segments[index].update_seg_header(field, val)?;
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "Invalid segment header index provided"))
        }

        Ok(())
    }

    pub fn write_segments(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        for i in (0..self.segments.len()).rev() {
            self.segments[i].write_segment(file_ptr)?;
        }
        Ok(())
    }


    pub fn write_exec_header(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        self.header.write_exec_header(file_ptr)?;
        Ok(())
    }

    pub fn write_header_tables(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        let sht_offset = match self.header.sht_offset() {
            SHTOffset::ThirtyTwo(off) => {
                off as u64
            }
            SHTOffset::SixtyFour(off) => {
                off
            }
        };

        let pht_offset = match self.header.pht_offset() {
            PHTOffset::ThirtyTwo(off) => {
                off as u64
            }
            PHTOffset::SixtyFour(off) => {
                off
            }
        };

        file_ptr.seek(SeekFrom::Start(sht_offset))?;

        for sec in &self.sections {
            match &sec.SH {
                SecHeader::ThirtyTwo(secheader) => {
                    match self.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            secheader.write_sec_header::<LittleEndian>(file_ptr)?;
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            secheader.write_sec_header::<BigEndian>(file_ptr)?;
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Cant write Elf"))
                    }
                }
                SecHeader::SixtyFour(secheader) => {
                    match self.header.data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            secheader.write_sec_header::<LittleEndian>(file_ptr)?;
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            secheader.write_sec_header::<BigEndian>(file_ptr)?;
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Cant write Elf"))
                    }
                }
            }
        }

        file_ptr.seek(SeekFrom::Start(pht_offset))?;

        for seg in &self.segments {
            match &seg.PH {
                ProgHeader::ThirtyTwo(progheader) => {
                    match self.header.data {
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
                    match self.header.data {
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

    pub fn write_sections(&self, file_ptr: &mut File) -> Result<(), std::io::Error> {
        for sec in &self.sections {
            sec.write_section(file_ptr)?;
        }
        Ok(())
    }
}
