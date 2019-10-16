/*
* Implementation of parsing/reading and writing
* ELF sections, string tables, and section headers.
*
*/

use std::fs::File;
use std::io::{Write, Read, Seek,SeekFrom};
use std::str;
use byteorder::*;


#[allow(non_snake_case)]
pub struct Section{
    pub SH: SecHeader,
    pub raw_bytes: Vec<u8>,
    pub name: String,
}

impl Section {

    /*Write the section to the file offset set in the section's header*/
    pub fn write_section(&self, file_ptr: &mut File) -> Result<(),std::io::Error>{
        file_ptr.seek(SeekFrom::Start(self.offset().into()))?;
        file_ptr.write(&mut self.raw_bytes.clone())?;
        Ok(())
    }

    pub fn name(&self)->String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn raw_bytes(&self)->Vec<u8>{
        self.raw_bytes.clone()
    }

    pub fn offset(&self)-> u64 {
        match &self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_offset as u64
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_offset
            }
        }
    }

    //TODO change to checked add for save addition in 32bit cases
    pub fn set_offset(&mut self, offset: u64) {
        match &mut self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_offset = offset as u32
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_offset = offset
            }
        }
    }

    //TODO change to checked add for save addition in 32bit cases
    pub fn increase_offset(&mut self, by_size: u64) {
        match &mut self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_offset += by_size as u32
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_offset += by_size
            }
        }
    }

    pub fn set_size(&mut self, size: u64) {
        match &mut self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_size = size as u32
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_size = size
            }
        }
    }


    pub fn update_sec_header(&mut self, field: String, val: u64) -> Result<(),std::io::Error>{
      //  where u32: core::convert::From<u64> + Copy{
        match &mut self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                sh.update_sec_header(field, val as u32)?;

            },
            SecHeader::SixtyFour(sh)=>{
                sh.update_sec_header(field, val)?;
            }
        }
        Ok(())
    }


    pub fn size(&self)-> u64 {
        match &self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_size as u64
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_size
            }
        }
    }


    /*
    NOTE: From ELf man page Some sections hold a table of fixed-sized entries, such as
                     a symbol table.  For such a section, this member gives the
                     size in bytes for each entry.  This member contains zero if
                     the section does not hold a table of fixed-size entries.
    */
    pub fn entsize(&self)-> u64 {
        match &self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_entsize as u64
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_entsize
            }
        }
    }

    pub fn name_idx(&self)-> u32 {
        match &self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_name
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_name
            }
        }
    }

    pub fn link_idx(&self)-> u32 {
        match &self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_link
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_link
            }
        }
    }

    pub fn shtype_as_u32(&self)-> u32 {
        match &self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_type
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_type
            }
        }
    }

    pub fn shtype_as_enum(&self)-> Result<SH_Type, std::io::Error> {
        match &self.SH{
            SecHeader::ThirtyTwo(sh)=>{
                return sh.sh_type()
            },
            SecHeader::SixtyFour(sh)=>{
                return sh.sh_type()
            }
        }
    }
}



#[derive(Clone, Debug)]
pub struct Strtab {
    pub name: String,
    pub strtab: Vec<u8>,
    pub parsed_strs: Vec<String>,
}

/*
* Implementation for parsing human-readable strings within a string table section.
*/
impl Strtab{

    pub fn parse_str_table(name: String, strtab_t: Vec<u8>) -> Strtab {
       // let mut strtab_t = vec![0; sh_size as usize];
       // rdr.read(&mut strtab_t);
        let chars: Vec<char> = strtab_t.clone().iter().filter_map(|b| {
            let str_char = char::from(*b);
            Some(str_char)
        }).collect();
        let new_str: String = chars.clone().into_iter().collect();
        let data_strs: Vec<String> =
            new_str.split("\u{0}").map(|s| s.to_string()).collect();

        println!("Parsed string! {:?}", data_strs);
        Strtab {
            name: name,
            strtab : strtab_t,
            parsed_strs: data_strs,
        }
    }

    pub fn new() -> Strtab {
        Strtab {
            name: " ".to_string(),
            strtab: Vec::new(),
            parsed_strs: Vec::new(),
        }
    }


}

/*
//TODO NEED T OHANDLE THE SPECIAL CASES THESE COVER!
/* special section indexes */
#define SHN_UNDEF	0
#define SHN_LORESERVE	0xff00
#define SHN_LOPROC	0xff00
#define SHN_HIPROC	0xff1f
#define SHN_LIVEPATCH	0xff20
#define SHN_ABS		0xfff1
#define SHN_COMMON	0xfff2
#define SHN_HIRESERVE	0xffff

*/


#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
/*Comments pulled from elf.h*/
pub enum SH_Type {
    SHT_NULL = 0,
    SHT_PROGBITS = 1,
    SHT_SYMTAB = 2,
    SHT_STRTAB = 3,
    SHT_RELA = 4,
    SHT_HASH = 5,
    SHT_DYNAMIC = 6,
    SHT_NOTE = 7,
    SHT_NOBITS = 8,
    SHT_REL = 9,
    SHT_SHLIB = 10,
    SHT_DYNSYM = 11,
    SHT_INIT_ARRAY	=  14,		/* Array of constructors */
    SHT_FINI_ARRAY=	  15,		/* Array of destructors */
    SHT_PREINIT_ARRAY= 16,		/* Array of pre-constructors */
    SHT_GROUP	=  17,		/* Section group */
    SHT_SYMTAB_SHNDX = 18,		/* Extended section indeces */
    SHT_NUM		=  19,		/* Number of defined types.  */
    SHT_LOOS=	  0x60000000,	/* Start OS-specific.  */
    SHT_GNU_ATTRIBUTES= 0x6ffffff5,	/* Object attributes.  */
    SHT_GNU_HASH	=  0x6ffffff6,	/* GNU-style hash table.  */
    SHT_GNU_LIBLIST	=  0x6ffffff7,	/* Prelink library list */
    SHT_CHECKSUM	=  0x6ffffff8,	/* Checksum for DSO content.  */

    SHT_GNU_verdef	 = 0x6ffffffd,	/* Version definition section.  */
    SHT_GNU_verneed	=  0x6ffffffe,	/* Version needs section.  */
    SHT_GNU_versym	=  0x6fffffff,	/* Version symbol table.  */

   /*NOTE: aliased discriminants not allowed in enums!*/
   // SHT_HIOS	=  SHT_GNU_versym,//0x6fffffff	/* End OS-specific type */
    SHT_LOPROC	=  0x70000000,	/* Start of processor-specific */
    SHT_HIPROC=	  0x7fffffff,	/* End of processor-specific */
    SHT_LOUSER=	  0x80000000,	/* Start of application-specific */
    SHT_HIUSER=	  0x8fffffff,	/* End of application-specific */

}


/*Comments pulled from elf.h*/
pub fn match_sh_type_as_str(sh_type: String) -> Result<u32, std::io::Error>{

    match sh_type.as_str() {
    "SHT_NULL" => Ok(0),
    "SHT_PROGBITS" => Ok(1),
    "SHT_SYMTAB" => Ok(2),
    "SHT_STRTAB" => Ok(3),
    "SHT_RELA" => Ok(4),
    "SHT_HASH" => Ok(5),
    "SHT_DYNAMIC" => Ok(6),
    "SHT_NOTE" => Ok(7),
    "SHT_NOBITS" => Ok(8),
    "SHT_REL" => Ok(9),
    "SHT_SHLIB" =>Ok(10),
    "SHT_DYNSYM" =>Ok(11),
    "SHT_INIT_ARRAY" => Ok(14),		/* Array of constructors */
    "SHT_FINI_ARRAY" =>	  Ok(15),		/* Array of destructors */
    "SHT_PREINIT_ARRAY" =>Ok(16),		/* Array of pre-constructors */
    "SHT_GROUP" =>  Ok(17),		/* Section group */
    "SHT_SYMTAB_SHNDX" =>Ok(18),		/* Extended section indeces */
    "SHT_NUM" => Ok(19),		/* Number of defined types.  */
    "SHT_LOOS" =>	  Ok(0x60000000),	/* Start OS-specific.  */
    "SHT_GNU_ATTRIBUTES" => Ok(0x6ffffff5),	/* Object attributes.  */
    "SHT_GNU_HASH" =>  Ok(0x6ffffff6),	/* GNU-style hash table.  */
    "SHT_GNU_LIBLIST" =>  Ok(0x6ffffff7),	/* Prelink library list */
    "SHT_CHECKSUM" => Ok(0x6ffffff8),	/* Checksum for DSO content.  */

    "SHT_GNU_verdef" => Ok(0x6ffffffd),	/* Version definition section.  */
    "SHT_GNU_verneed" =>  Ok(0x6ffffffe),	/* Version needs section.  */
    "SHT_GNU_versym" => Ok(0x6fffffff),	/* Version symbol table.  */

    /*NOTE: aliased discriminants not allowed in enums!*/
    "SHT_HIOS" =>  Ok(0x6fffffff),	/* End OS-specific type */
    "SHT_LOPROC" =>  Ok(0x70000000),	/* Start of processor-specific */
    "SHT_HIPROC" =>	  Ok(0x7fffffff),	/* End of processor-specific */
    "SHT_LOUSER" =>	  Ok(0x80000000),	/* Start of application-specific */
    "SHT_HIUSER" =>  Ok(0x8fffffff),	/* End of application-specific */
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid sh_type replacement"))
    }
}

#[allow(non_camel_case_types)]
pub enum SH_Flags {
    SHF_WRITE = 0x1,
    SHF_ALLOC = 0x2,
    SHF_EXECINSTR =0x4,
    SHF_MASKPROC = 0xf0000000,
    SHF_MERGE = 0x10,
    SHF_STRINGS = 0x20,
    SHF_INFO_LINK = 0x40,
    SHF_LINK_ORDER = 0x80,
    SHF_OS_NONCONFORMING = 0x100,
    SHF_GROUP = 0x200,
    SHF_TLS = 0x400,
    SHF_COMPRESSED = 0x800,
    SHF_MASKOS = 0x0ff00000,
    SHF_RELA_LIVEPATCH = 0x00100000,
    SHF_RO_AFTER_INIT = 0x00200000,

}

/*
* /* special section indexes */
#define SHN_UNDEF	0
#define SHN_LORESERVE	0xff00
#define SHN_LOPROC	0xff00
#define SHN_HIPROC	0xff1f
#define SHN_LIVEPATCH	0xff20
#define SHN_ABS		0xfff1
#define SHN_COMMON	0xfff2
#define SHN_HIRESERVE	0xffff
*
*/


/* TODO need to look up any other potentially supported flags, likely processor specific?*/
/*NOTE: Caller will need to downcast to u32 for 32bit section headers*/
pub fn match_sh_flag_as_str(sh_flag: String) -> Result<u64, std::io::Error> {
    match sh_flag.as_str() {
        "SHF_WRITE" =>  Ok(0x1),
        "SHF_ALLOC" =>  Ok(0x2),
        "SHF_EXECINSTR" =>  Ok(0x4),
        "SHF_MASKPROC" =>   Ok(0xf0000000),
        "SHF_MERGE" =>  Ok(0x10),
        "SHF_STRINGS" =>  Ok(0x20),
        "SHF_INFO_LINK" =>  Ok(0x40),
        "SHF_LINK_ORDER" =>  Ok(0x80),
        "SHF_OS_NONCONFORMING" =>  Ok(0x100),
        "SHF_GROUP" =>  Ok(0x200),
        "SHF_TLS" =>  Ok(0x400),
        "SHF_COMPRESSED" =>  Ok(0x800),
        "SHF_MASKOS" =>  Ok(0x0ff00000),
        "SHF_RELA_LIVEPATCH" =>	Ok(0x00100000),
        "SHF_RO_AFTER_INIT" => Ok(0x00200000),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid sh_flag replacement"))
    }
}

        #[derive(Clone, Debug)]
pub enum SecHeader{
    ThirtyTwo(SecHeader32),
    SixtyFour(SecHeader64),
}

#[derive(Debug)]
pub struct SecHeader32{
    pub sh_name: u32,
    pub sh_type: u32, //SH_Type
    pub sh_flags: u32, //SH_Flags
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

#[derive(Debug)]
pub struct SecHeader64{
    pub sh_name: u32,
    pub sh_type: u32, //SH_Type
    pub sh_flags: u64, //SH_Flags
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Copy for SecHeader32{}

impl Clone for SecHeader32{

    fn clone(&self)->SecHeader32{
        SecHeader32{
            sh_name: self.sh_name,
            sh_type: self.sh_type,
            sh_flags: self.sh_flags,
            sh_addr: self.sh_addr,
            sh_offset: self.sh_offset,
            sh_size: self.sh_size,
            sh_link: self.sh_link,
            sh_info: self.sh_info,
            sh_addralign:self.sh_addralign,
            sh_entsize: self.sh_entsize,
        }
    }
}


/*
* Section header implementation, for parsing, writing, and creating
* new sections as needed.
*/
impl SecHeader32{
    pub fn default()-> SecHeader32{
        SecHeader32{
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }

    pub fn new( name: u32, shtype: u32, flags: u32, addr: u32,
                offset: u32, size: u32, link: u32, info: u32,
                addralign: u32, entsize: u32, ) -> SecHeader32 {
        SecHeader32 {
            sh_name: name,
            sh_type: shtype,
            sh_flags: flags,
            sh_addr: addr,
            sh_offset: offset,
            sh_size: size,
            sh_link: link,
            sh_info: info,
            sh_addralign: addralign,
            sh_entsize: entsize,
        }
    }



    pub fn parse_sec32_header<R, B>(rdr: &mut R,) -> Result<SecHeader32,std::io::Error>
        where R: Read + Seek+ ReadBytesExt, B: ByteOrder {
        let name = rdr.read_u32::<B>()?;
        let shtype = rdr.read_u32::<B>()?;
        let flags = rdr.read_u32::<B>()?;
        let addr = rdr.read_u32::<B>()?;
        let offset = rdr.read_u32::<B>()?;
        let size = rdr.read_u32::<B>()?;
        let link = rdr.read_u32::<B>()?;
        let info = rdr.read_u32::<B>()?;
        let addralign = rdr.read_u32::<B>()?;
        let entsize = rdr.read_u32::<B>()?;

        Ok(SecHeader32 {
            sh_name: name,
            sh_type: shtype, //SH_Type
            sh_flags: flags,
            sh_addr: addr,
            sh_offset: offset,
            sh_size: size,
            sh_link: link,
            sh_info: info,
            sh_addralign: addralign,
            sh_entsize: entsize,
        })
    }


    /*Update a field within the section header*/
    pub fn update_sec_header(&mut self, field: String, val: u32) -> Result<(), std::io::Error>{
        match field.as_str() {
            "sh_name" =>  self.sh_name = val,
            "sh_type" => self.sh_type = val,
            "sh_flags" => self.sh_flags =val,
            "sh_addr" => self.sh_addr=val,
            "sh_offset" => self.sh_offset=val,
            "sh_size" => self.sh_size=val,
            "sh_link" => self.sh_link=val,
            "sh_info" => self.sh_info=val,
            "sh_addralign" => self.sh_addralign=val,
            "sh_entsize" => self.sh_entsize=val,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid section header field provided"))
        };
        Ok(())
    }

    pub fn write_sec_header<B>(&self, wrtr: &mut File) -> Result<(),std::io::Error>
        where B: ByteOrder {
        wrtr.write_u32::<B>(self.sh_name)?;
        wrtr.write_u32::<B>(self.sh_type)?;
        wrtr.write_u32::<B>(self.sh_flags)?;
        wrtr.write_u32::<B>(self.sh_addr)?;
        wrtr.write_u32::<B>(self.sh_offset)?;
        wrtr.write_u32::<B>(self.sh_size)?;
        wrtr.write_u32::<B>(self.sh_link)?;
        wrtr.write_u32::<B>(self.sh_info)?;
        wrtr.write_u32::<B>(self.sh_addralign)?;
        wrtr.write_u32::<B>(self.sh_entsize)?;
        Ok(())
    }


    pub fn offset(&self)-> u32 {
        self.sh_offset
    }

    pub fn size(&self)->u32 {
        self.sh_size
    }

    pub fn entsize(&self)->u32 {
        self.sh_entsize
    }


    /*Parse the value and return the associated enum*/
    pub fn sh_type(&self)->Result<SH_Type, std::io::Error> {
        match self.sh_type {

            0 => Ok(SH_Type::SHT_NULL),
            1=> Ok(SH_Type::SHT_PROGBITS) ,
            2 => Ok(SH_Type::SHT_SYMTAB) ,
            3 => Ok(SH_Type::SHT_STRTAB) ,
            4=> Ok(SH_Type::SHT_RELA) ,
            5=> Ok(SH_Type::SHT_HASH) ,
            6=> Ok(SH_Type::SHT_DYNAMIC) ,
            7 => Ok(SH_Type::SHT_NOTE) ,
            8 => Ok(SH_Type::SHT_NOBITS) ,
            9 => Ok(SH_Type::SHT_REL) ,
            10 => Ok(SH_Type::SHT_SHLIB) ,
            11 => Ok(SH_Type::SHT_DYNSYM) ,
            14 => Ok(SH_Type::SHT_INIT_ARRAY)	,		/* Array of constructors */
            15 => Ok(SH_Type::SHT_FINI_ARRAY),		/* Array of destructors */
            16 => Ok(SH_Type::SHT_PREINIT_ARRAY),		/* Array of pre-constructors */
            17 => Ok(SH_Type::SHT_GROUP)	,		/* Section group */
            18 => Ok(SH_Type::SHT_SYMTAB_SHNDX) ,		/* Extended section indeces */
            19  => Ok(SH_Type::SHT_NUM)		,		/* Number of defined types.  */
            /* Start OS-specific.  */
            0x60000000 => Ok(SH_Type::SHT_LOOS),
            0x6ffffff5 => Ok(SH_Type::SHT_GNU_ATTRIBUTES),	/* Object attributes.  */
            0x6ffffff6 => Ok(SH_Type::SHT_GNU_HASH)	 ,	/* GNU-style hash table.  */
            0x6ffffff7 => Ok(SH_Type::SHT_GNU_LIBLIST)	 ,	/* Prelink library list */
            0x6ffffff8 => Ok(SH_Type::SHT_CHECKSUM)	,	/* Checksum for DSO content.  */

            0x6ffffffd => Ok(SH_Type::SHT_GNU_verdef)	 ,	/* Version definition section.  */
            0x6ffffffe => Ok(SH_Type::SHT_GNU_verneed)  ,	/* Version needs section.  */
            0x6fffffff => Ok(SH_Type::SHT_GNU_versym)  ,	/* Version symbol table.  */
            // SHT_HIOS	=  SHT_GNU_versym,//0x6fffffff	/* End OS-specific type */
            0x70000000 => Ok(SH_Type::SHT_LOPROC)  ,	/* Start of processor-specific */
            0x7fffffff => Ok(SH_Type::SHT_HIPROC)	 ,	/* End of processor-specific */
            0x80000000 => Ok(SH_Type::SHT_LOUSER),	/* Start of application-specific */
            0x8fffffff => Ok(SH_Type::SHT_HIUSER),	/* End of application-specific */
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Elf not supported"))
        }
    }
}

impl Copy for SecHeader64{}

impl Clone for SecHeader64{

    fn clone(&self)->SecHeader64{
        SecHeader64{
            sh_name: self.sh_name,
            sh_type: self.sh_type,
            sh_flags: self.sh_flags,
            sh_addr: self.sh_addr,
            sh_offset: self.sh_offset,
            sh_size: self.sh_size,
            sh_link: self.sh_link,
            sh_info: self.sh_info,
            sh_addralign:self.sh_addralign,
            sh_entsize: self.sh_entsize,
        }
    }
}


/*Section header implementation, for parsing, writing, and creating
* new sections as needed. */
impl SecHeader64{

    pub fn default()-> SecHeader64 {
        SecHeader64{
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }

    pub fn new( name: u32, shtype: u32, flags: u64, addr: u64,
                offset: u64, size: u64, link: u32, info: u32,
                addralign: u64, entsize: u64, ) -> SecHeader64 {
        SecHeader64 {
            sh_name: name,
            sh_type: shtype,
            sh_flags: flags,
            sh_addr: addr,
            sh_offset: offset,
            sh_size: size,
            sh_link: link,
            sh_info: info,
            sh_addralign: addralign,
            sh_entsize: entsize,
        }
    }

    pub fn parse_sec64_header<R, B>(rdr: &mut R,) -> Result<SecHeader64,std::io::Error>
        where R: Read + Seek+ ReadBytesExt, B: ByteOrder {
        let name = rdr.read_u32::<B>()?;
        let shtype = rdr.read_u32::<B>()?;
        let flags = rdr.read_u64::<B>()?;
        let addr = rdr.read_u64::<B>()?;
        let offset = rdr.read_u64::<B>()?;
        let size = rdr.read_u64::<B>()?;
        let link = rdr.read_u32::<B>()?;
        let info = rdr.read_u32::<B>()?;
        let addralign = rdr.read_u64::<B>()?;
        let entsize = rdr.read_u64::<B>()?;

        Ok(SecHeader64 {
            sh_name: name,
            sh_type: shtype, //SH_Type
            sh_flags: flags,
            sh_addr: addr,
            sh_offset: offset,
            sh_size: size,
            sh_link: link,
            sh_info: info,
            sh_addralign: addralign,
            sh_entsize: entsize,
        })
    }

    /*Update the section header with new value for given field*/
    pub fn update_sec_header(&mut self, field: String, val: u64) -> Result<(),std::io::Error> {
        match field.as_ref() {
            "sh_name" =>  self.sh_name = val as u32,//super::as_u32(val),
            "sh_type" => self.sh_type = val as u32,//super::as_u32(val),
            "sh_flags" => self.sh_flags =val,
            "sh_addr" => self.sh_addr=val,
            "sh_offset" => self.sh_offset=val,
            "sh_size" => self.sh_size=val,
            "sh_link" => self.sh_link=val as u32,//super::as_u32(val),
            "sh_info" => self.sh_info=val as u32, //super::as_u32(val),
            "sh_addralign" => self.sh_addralign=val,
            "sh_entsize" => self.sh_entsize=val,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                "Invalid section header field provided"))
        };
        Ok(())
    }

    pub fn write_sec_header<B>(&self, wrtr: &mut File ) -> Result<(),std::io::Error>
        where B:  ByteOrder {
        wrtr.write_u32::<B>(self.sh_name)?;
        wrtr.write_u32::<B>(self.sh_type)?;
        wrtr.write_u64::<B>(self.sh_flags)?;
        wrtr.write_u64::<B>(self.sh_addr)?;
        wrtr.write_u64::<B>(self.sh_offset)?;
        wrtr.write_u64::<B>(self.sh_size)?;
        wrtr.write_u32::<B>(self.sh_link)?;
        wrtr.write_u32::<B>(self.sh_info)?;
        wrtr.write_u64::<B>(self.sh_addralign)?;
        wrtr.write_u64::<B>(self.sh_entsize)?;
        Ok(())
    }

    pub fn offset(&self)-> u64 {
        self.sh_offset
    }

    pub fn size(&self)->u64 {
        self.sh_size
    }

    pub fn entsize(&self)->u64 {
        self.sh_entsize
    }

    pub fn sh_type(&self)->Result<SH_Type, std::io::Error> {
        match self.sh_type {

            0 => Ok(SH_Type::SHT_NULL),
            1=> Ok(SH_Type::SHT_PROGBITS) ,
            2 => Ok(SH_Type::SHT_SYMTAB) ,
            3 => Ok(SH_Type::SHT_STRTAB) ,
            4=> Ok(SH_Type::SHT_RELA) ,
            5=> Ok(SH_Type::SHT_HASH) ,
            6=> Ok(SH_Type::SHT_DYNAMIC) ,
            7 => Ok(SH_Type::SHT_NOTE) ,
            8 => Ok(SH_Type::SHT_NOBITS) ,
            9 => Ok(SH_Type::SHT_REL) ,
            10 => Ok(SH_Type::SHT_SHLIB) ,
            11 => Ok(SH_Type::SHT_DYNSYM) ,
            14 => Ok(SH_Type::SHT_INIT_ARRAY)	,		/* Array of constructors */
            15 => Ok(SH_Type::SHT_FINI_ARRAY),		/* Array of destructors */
            16 => Ok(SH_Type::SHT_PREINIT_ARRAY),		/* Array of pre-constructors */
            17 => Ok(SH_Type::SHT_GROUP)	,		/* Section group */
            18 => Ok(SH_Type::SHT_SYMTAB_SHNDX) ,		/* Extended section indeces */
            19  => Ok(SH_Type::SHT_NUM)		,		/* Number of defined types.  */
            /* Start OS-specific.  */
            0x60000000 => Ok(SH_Type::SHT_LOOS),
            0x6ffffff5 => Ok(SH_Type::SHT_GNU_ATTRIBUTES),	/* Object attributes.  */
            0x6ffffff6 => Ok(SH_Type::SHT_GNU_HASH)	 ,	/* GNU-style hash table.  */
            0x6ffffff7 => Ok(SH_Type::SHT_GNU_LIBLIST)	 ,	/* Prelink library list */
            0x6ffffff8 => Ok(SH_Type::SHT_CHECKSUM)	,	/* Checksum for DSO content.  */

            0x6ffffffd => Ok(SH_Type::SHT_GNU_verdef)	 ,	/* Version definition section.  */
            0x6ffffffe => Ok(SH_Type::SHT_GNU_verneed)  ,	/* Version needs section.  */
            0x6fffffff => Ok(SH_Type::SHT_GNU_versym)  ,	/* Version symbol table.  */
            // SHT_HIOS	=  SHT_GNU_versym,//0x6fffffff	/* End OS-specific type */
            0x70000000 => Ok(SH_Type::SHT_LOPROC)  ,	/* Start of processor-specific */
            0x7fffffff => Ok(SH_Type::SHT_HIPROC)	 ,	/* End of processor-specific */
            0x80000000 => Ok(SH_Type::SHT_LOUSER),	/* Start of application-specific */
            0x8fffffff => Ok(SH_Type::SHT_HIUSER),	/* End of application-specific */
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "sh_type not supported"))
        }
    }

}
