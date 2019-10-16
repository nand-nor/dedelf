use std::fs::File;
use std::path::PathBuf;

use std::io::{Read, Seek, SeekFrom, Write};


use byteorder::*;
use crate::header::PHTOffset;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Segment{
    pub PH: ProgHeader,
    pub raw_bytes: Vec<u8>,
}

impl Segment{

    /*Write the section to the file pointer -- ptr must already be set to correct byte offset*/
    pub fn write_segment(&self, file_ptr: &mut File) ->Result<(),std::io::Error>{

        let _off = self.offset();

        let offset: u64 = match _off{
            PHTOffset::ThirtyTwo(offset)=>{offset as u64},
            PHTOffset::SixtyFour(offset)=>{offset},
        };
        file_ptr.seek(SeekFrom::Start(offset.into()))?;

        //TODO does this really require cloning the bytes?
        file_ptr.write(&mut self.raw_bytes.clone())?;
        Ok(())
    }

    pub fn set_bytes(&mut self, new_bytes: Vec<u8>){
        self.raw_bytes = new_bytes;
    }

    pub fn offset(&self)->PHTOffset{

        match &self.PH{
            ProgHeader::ThirtyTwo(ph)=>{
                return PHTOffset::ThirtyTwo(ph.p_offset)
            },
            ProgHeader::SixtyFour(ph)=>{
                return PHTOffset::SixtyFour(ph.p_offset)
            }
        }
    }


    pub fn increase_offset(&mut self, by_size: u64){

        match &mut self.PH{
            ProgHeader::ThirtyTwo(ph)=>{
                ph.p_offset += by_size as u32
            },
            ProgHeader::SixtyFour(ph)=>{
                ph.p_offset += by_size
            }
        }
    }

    pub fn increase_size(&mut self, by_size: u64) {

        match &mut self.PH{
            ProgHeader::ThirtyTwo(ph)=>{
                //TODO safety check for adding the u32's
                ph.update_seg_header("p_filesz".to_string(), ph.p_filesz+by_size as u32).unwrap();
                ph.update_seg_header("p_memsz".to_string(), ph.p_memsz+by_size as u32).unwrap();

            },
            ProgHeader::SixtyFour(ph)=>{
                //TODO safety check for adding the u32's
                ph.update_seg_header("p_filesz".to_string(), ph.p_filesz+by_size).unwrap();
                ph.update_seg_header("p_memsz".to_string(), ph.p_memsz+by_size).unwrap();
            }
        }
    }


    pub fn update_seg_header(&mut self, field: String, val: u64) -> Result<(),std::io::Error> {
        match &mut self.PH{
            ProgHeader::ThirtyTwo(ph)=>{
                ph.update_seg_header(field, val as u32)?;

            },
            ProgHeader::SixtyFour(ph)=>{
                ph.update_seg_header(field, val)?;
            }
        }
        Ok(())
    }

    pub fn mem_size(&self)->u64{
        match &self.PH{
            ProgHeader::ThirtyTwo(ph)=>{
                return ph.p_memsz as u64
            },
            ProgHeader::SixtyFour(ph)=>{
                return ph.p_memsz
            }
        }
    }

    pub fn file_size(&self)->u64{
        match &self.PH{
            ProgHeader::ThirtyTwo(ph)=>{
                return ph.p_filesz as u64
            },
            ProgHeader::SixtyFour(ph)=>{
                return ph.p_filesz
            }
        }
    }

}


#[derive(Clone, Debug)]
pub enum ProgHeader{
    ThirtyTwo(ProgHeader32),
    SixtyFour(ProgHeader64),
}


/*
    NOTE! From ELf man page
    ph_align This member holds the value to which the segments are
                 aligned in memory and in the file.  Loadable process seg‐
                 ments must have congruent values for p_vaddr and p_offset,
                 modulo the page size.  Values of zero and one mean no
                 alignment is required.  Otherwise, p_align should be a pos‐
                 itive, integral power of two, and p_vaddr should equal
                 p_offset, modulo p_align.

*/

#[derive(Copy, Clone, Debug)]
pub struct ProgHeader32 {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,

}



#[derive(Copy, Clone, Debug)]
pub struct ProgHeader64{
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl ProgHeader32 {

    /*Create a new ProgHeader given a set of data */
    pub fn new(ptype: u32, offset: u32, vaddr: u32, paddr:u32, filesz: u32,
               memsz: u32, flags: u32, align: u32, ) -> ProgHeader32 {
        ProgHeader32 {
            p_type: ptype,
            p_offset: offset,
            p_vaddr: vaddr,
            p_paddr: paddr,
            p_filesz: filesz,
            p_memsz: memsz,
            p_flags: flags,
            p_align: align,
        }
    }

    /*Given a pointer to a file location (rdr), return a ProgHeader
    * struct populated with the read-in bytes*/
    pub fn parse_prog32_header<R, B>(rdr: &mut R,) -> Result<ProgHeader32,std::io::Error>
        where R: Read + Seek+ ReadBytesExt, B: ByteOrder {

        let ptype = rdr.read_u32::<B>()?;
        let offset = rdr.read_u32::<B>()?;
        let vaddr = rdr.read_u32::<B>()?;
        let paddr = rdr.read_u32::<B>()?;
        let filesz = rdr.read_u32::<B>()?;
        let memsz = rdr.read_u32::<B>()?;
        let flags = rdr.read_u32::<B>()?;
        let align = rdr.read_u32::<B>()?;
        Ok(ProgHeader32 {
            p_type: ptype,
            p_offset: offset,
            p_vaddr: vaddr,
            p_paddr: paddr,
            p_filesz: filesz,
            p_memsz: memsz,
            p_flags: flags,
            p_align: align,

        })
    }


    pub fn update_seg_header(&mut self, field: String, val: u32) -> Result<(), std::io::Error>{
        match field.as_str() {
            "p_type" => self.p_type = val,
            "p_offset" => self.p_offset = val,
            "p_vaddr" => self.p_vaddr = val,
            "p_paddr" => self.p_paddr = val,
            "p_filesz" => self.p_filesz = val,
            "p_memsz" => self.p_memsz = val,
            "p_flags" => self.p_flags = val,
            "p_align" => self.p_align = val,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                "Invalid program header field provided"))
        };
        Ok(())
    }



    /*Assumes that the file pointer (wrtr) has already been
    * moved to the correct position and that all header data
    * has been adjusted according to the needs of the segment
    * e.g. if this is a new segment, then all the necessary
    * adjustments and calculatons have already been done*/

    pub fn write_prog_header<B>(&self, wrtr: &mut File ) -> Result<(),std::io::Error>
        where B: ByteOrder {
        wrtr.write_u32::<B>(self.p_type)?;
        wrtr.write_u32::<B>(self.p_offset)?;
        wrtr.write_u32::<B>(self.p_vaddr)?;
        wrtr.write_u32::<B>(self.p_paddr)?;
        wrtr.write_u32::<B>(self.p_filesz)?;
        wrtr.write_u32::<B>(self.p_memsz)?;
        wrtr.write_u32::<B>(self.p_flags)?;
        wrtr.write_u32::<B>(self.p_align)?;
        Ok(())
    }

    pub fn offset(&self)->u32{
        self.p_offset
    }

    pub fn mem_size(&self)->u32{
        self.p_memsz
    }

    pub fn filesize(&self)->u32 {
        self.p_filesz
    }

}

impl ProgHeader64 {

    /*Create a new ProgHeader given a set of data */
    pub fn new(ptype: u32, offset: u64, vaddr: u64, paddr:u64, filesz: u64,
               memsz: u64, flags: u32, align: u64, ) -> ProgHeader64 {
        ProgHeader64 {
            p_type: ptype,
            p_offset: offset,
            p_vaddr: vaddr,
            p_paddr: paddr,
            p_filesz: filesz,
            p_memsz: memsz,
            p_flags: flags,
            p_align: align,
        }
    }

    /*Given a pointer to a file location (rdr), return a ProgHeader
    * struct populated with the read-in bytes*/
    pub fn parse_prog64_header<R, B>(rdr: &mut R,) -> Result<ProgHeader64,std::io::Error>
        where R: Read + Seek+ ReadBytesExt, B: ByteOrder {

        let ptype = rdr.read_u32::<B>()?;
        let flags = rdr.read_u32::<B>()?;
        let offset = rdr.read_u64::<B>()?;
        let vaddr = rdr.read_u64::<B>()?;
        let paddr = rdr.read_u64::<B>()?;
        let filesz = rdr.read_u64::<B>()?;
        let memsz = rdr.read_u64::<B>()?;
        let align = rdr.read_u64::<B>()?;
        Ok(ProgHeader64 {
            p_type: ptype,
            p_flags: flags,
            p_offset: offset,
            p_vaddr: vaddr,
            p_paddr: paddr,
            p_filesz: filesz,
            p_memsz: memsz,
            p_align: align,

        })
    }

    /*
    * Assumes that the file pointer (wrtr) has already been
    * moved to the correct position and that all header data
    * has been adjusted according to the needs of the segment
    * e.g. if this is a new segment, then all the necessary
    * adjustments to offsets/sizes and calculations have already
    * been made to the prog header
    */
    pub fn write_prog_header<B>(&self, wrtr: &mut File ) -> Result<(), std::io::Error>
        where B: ByteOrder {
        wrtr.write_u32::<B>(self.p_type)?;
        wrtr.write_u32::<B>(self.p_flags)?;
        wrtr.write_u64::<B>(self.p_offset)?;
        wrtr.write_u64::<B>(self.p_vaddr)?;
        wrtr.write_u64::<B>(self.p_paddr)?;
        wrtr.write_u64::<B>(self.p_filesz)?;
        wrtr.write_u64::<B>(self.p_memsz)?;
        wrtr.write_u64::<B>(self.p_align)?;
        Ok(())
    }

    pub fn update_seg_header(&mut self, field: String, val: u64)-> Result<(), std::io::Error> {
        match field.as_str() {
            "p_type" =>  self.p_type = val as u32,
            "p_offset" => self.p_offset = val,
            "p_vaddr" => self.p_vaddr =val,
            "p_paddr" => self.p_paddr=val,
            "p_filesz" => self.p_filesz=val,
            "p_memsz" => self.p_memsz=val,
            "p_flags" => self.p_flags=val as u32,
            "p_align" => self.p_align=val,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                "Invalid program header field provided"))
        };
        Ok(())
    }

    pub fn offset(&self)->u64{
        self.p_offset
    }

    pub fn mem_size(&self)->u64{
        self.p_memsz
    }

    pub fn filesize(&self)->u64 {
        self.p_filesz
    }

}

#[allow(non_camel_case_types, non_snake_case)]
pub enum P_flag{
    PF_X = 0x1,
    PF_W = 0x2,
    PF_R = 0x4,
}

pub fn match_p_flag_as_str(p_flag: String) -> Result<u32, std::io::Error> {
    let trimmed = p_flag.trim_start_matches("0x");
    let check = u32::from_str_radix(trimmed, 16);
    if check.is_err() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                       "Invalid p_flag value provided"))
    }
    let val = check.unwrap();
    if val == 0 || val > 7 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                       "Invalid p_flag value provided: valid values \
                                       include any bit combination of anding the values 1,2, \
                                       and/or 4,"))
    } else {
        return Ok(val)
    }
}

/*comments pulled from elf.h*/
#[allow(non_camel_case_types, non_snake_case)]
pub enum PT_type {
    PT_NULL   =  0,       /* Program header table entry unused */
    PT_LOAD  =   1,       /* Loadable program segment */
    PT_DYNAMIC=  2,       /* Dynamic linking information */
    PT_INTERP=   3,       /* Program interpreter */
    PT_NOTE  =   4,       /* Auxiliary information */
    PT_SHLIB =   5,       /* Reserved */
    PT_PHDR  =   6,       /* Entry for header table itself */
    PT_TLS   =   7,       /* Thread-local storage segment */
    PT_NUM   =   8,       /* Number of defined types */
    PT_LOOS =    0x60000000,  /* Start of OS-specific */
    PT_GNU_EH_FRAME= 0x6474e550,  /* GCC .eh_frame_hdr segment */
    PT_GNU_STACK =   0x6474e551,  /* Indicates stack executability */
    PT_GNU_RELRO=    0x6474e552,  /* Read-only after relocation */
    PT_LOSUNW  = 0x6ffffffa,
     PT_SUNWSTACK=    0x6ffffffb,  /* Stack segment */
    //   PT_HISUNW  = 0x6fffffff,
    PT_HIOS   =  0x6fffffff,  /* End of OS-specific */
    PT_LOPROC=   0x70000000,  /* Start of processor-specific */
    PT_HIPROC=   0x7fffffff,  /* End of processor-specific */
}

//TODO make sure all possible p types are represented here!
/*comments pulled from elf.h*/
pub fn match_p_type_as_str(p_type: String) -> Result<u32, std::io::Error> {
    match p_type.as_str() {
        "PT_NULL" => Ok(0),       /* Program header table entry unused */
        "PT_LOAD" => Ok(1),       /* Loadable program segment */
        "PT_DYNAMIC" => Ok(2),       /* Dynamic linking information */
        "PT_INTERP" => Ok(3),       /* Program interpreter */
        "PT_NOTE" => Ok(4),       /* Auxiliary information */
        "PT_SHLIB" => Ok(5),       /* Reserved */
        "PT_PHDR" => Ok(6),       /* Entry for header table itself */
        "PT_TLS" => Ok(7),       /* Thread-local storage segment */
        "PT_NUM" => Ok(8),       /* Number of defined types */
        "PT_LOOS" => Ok(0x60000000),  /* Start of OS-specific */
        "PT_GNU_EH_FRAME" => Ok(0x6474e550),  /* GCC .eh_frame_hdr segment */
        "PT_GNU_STACK" => Ok(0x6474e551),  /* Indicates stack executability */
        "PT_GNU_RELRO" => Ok(0x6474e552),  /* Read-only after relocation */
        "PT_LOSUNW" => Ok(0x6ffffffa),
        "PT_SUNWSTACK" => Ok(0x6ffffffb),  /* Stack segment */
        "PT_HISUNW" => Ok(0x6fffffff),
        "PT_HIOS" => Ok(0x6fffffff),  /* End of OS-specific */
        "PT_LOPROC" => Ok(0x70000000),  /* Start of processor-specific */
        "PT_HIPROC" => Ok(0x7fffffff),  /* End of processor-specific */
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            "Invalid program header type replacement provided"))
    }
}


//TODO need to deal with ph extended numbering
/*

/*
 * Extended Numbering
 *
 * If the real number of program header table entries is larger than
 * or equal to PN_XNUM(0xffff), it is set to sh_info field of the
 * section header at index 0, and PN_XNUM is set to e_phnum
 * field. Otherwise, the section header at index 0 is zero
 * initialized, if it exists.
 *
 * Specifications are available in:
 *
 * - Oracle: Linker and Libraries.
 *   Part No: 817–1984–19, August 2011.
 *   http://docs.oracle.com/cd/E18752_01/pdf/817-1984.pdf
 *
 * - System V ABI AMD64 Architecture Processor Supplement
 *   Draft Version 0.99.4,
 *   January 13, 2010.
 *   http://www.cs.washington.edu/education/courses/cse351/12wi/supp-docs/abi.pdf
 */
#define PN_XNUM 0xffff
*/
