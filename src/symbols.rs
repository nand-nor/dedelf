use std::io::Read;
use byteorder::*;

use crate::header::*;


/*TODO: have a symbol trait and implement for dynamic and non-dynamic symbols,
*  since they are pretty much the same thing...
*/




#[derive(Clone, Debug)]
pub enum Symbol{
    ThirtyTwo(Symbol32),
    SixtyFour(Symbol64)
}

#[derive(Clone, Debug)]
pub enum DynSymbol{
    ThirtyTwo(DynSymbol32),
    SixtyFour(DynSymbol64)
}



#[derive(Clone, Debug)]
pub struct Symbol32 {
    pub st_name: u32,
    pub  st_value: u32,
    pub  st_size: u32,
    pub  st_info: u8,
    pub  st_other: u8,
    pub  st_shndx: u16,
}

/*Implementation for parsing symbols within sym sections. This will
* be used to overwrite symbols and update symbol entries in the
* ELF
*/
impl Symbol32 {
    pub fn parse_symbol<R, T: ByteOrder>(rdr: &mut R) -> Result<Symbol32, std::io::Error>
        where R: Read  {
        let name = rdr.read_u32::<T>()?;
        let value = rdr.read_u32::<T>()?;
        let size = rdr.read_u32::<T>()?;
        let mut info = [0; 1];
        rdr.read_exact(&mut info)?;
        let mut other = [0; 1];
        rdr.read_exact(&mut other)?;
        let shndx = rdr.read_u16::<T>()?;


        Ok(Symbol32 {
            st_name: name,
            st_value: value,
            st_size: size,
            st_info: info[0],
            st_other: other[0],
            st_shndx: shndx,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Symbol64 {
    pub st_name: u32,
    pub  st_info: u8,
    pub  st_other: u8,
    pub  st_shndx: u16,
    pub  st_value: u64,
    pub  st_size: u64,

}

/*Implementation for parsing symbols within sym sections. This will
* be used to overwrite symbols and update symbol entries in the
* ELF
*/
impl Symbol64 {
    pub fn parse_symbol<R, T: ByteOrder>(rdr: &mut R) -> Result<Symbol64, std::io::Error>
        where R: Read  {
        let name = rdr.read_u32::<T>()?;
        let mut info = [0; 1];
        rdr.read_exact(&mut info)?;
        let mut other = [0; 1];
        rdr.read_exact(&mut other)?;
        let shndx = rdr.read_u16::<T>()?;
        let value = rdr.read_u64::<T>()?;
        let size = rdr.read_u64::<T>()?;

        Ok(Symbol64 {
            st_name: name,
            st_value: value,
            st_size: size,
            st_info: info[0],
            st_other: other[0],
            st_shndx: shndx,
        })
    }
}



#[derive(Clone, Debug)]
pub struct DynSymbol32 {
    pub st_name: u32,
    pub  st_value: u32,
    pub  st_size: u32,
    pub  st_info: u8,
    pub  st_other: u8,
    pub  st_shndx: u16,
}

/*Implementation for parsing symbols within sym sections. This will
* be used to overwrite symbols and update symbol entries in the
* ELF
*/
impl DynSymbol32 {
    pub fn parse_symbol<R, T: ByteOrder>(rdr: &mut R) -> Result<DynSymbol32, std::io::Error>
        where R: Read  {
        let name = rdr.read_u32::<T>()?;
        let value = rdr.read_u32::<T>()?;
        let size = rdr.read_u32::<T>()?;
        let mut info = [0; 1];
        rdr.read_exact(&mut info)?;
        let mut other = [0; 1];
        rdr.read_exact(&mut other)?;
        let shndx = rdr.read_u16::<T>()?;

        Ok(DynSymbol32 {
            st_name: name,
            st_value: value,
            st_size: size,
            st_info: info[0],
            st_other: other[0],
            st_shndx: shndx,
        })
    }
}

#[derive(Clone, Debug)]
pub struct DynSymbol64 {
    pub st_name: u32,
    pub  st_info: u8,
    pub  st_other: u8,
    pub  st_shndx: u16,
    pub  st_value: u64,
    pub  st_size: u64,

}

/*Implementation for parsing symbols within sym sections. This will
* be used to overwrite symbols and update symbol entries in the
* ELF
*/
impl DynSymbol64 {
    pub fn parse_symbol<R, T: ByteOrder>(rdr: &mut R) -> Result<DynSymbol64, std::io::Error>
        where R: Read  {
        let name = rdr.read_u32::<T>()?;
        let mut info = [0; 1];
        rdr.read_exact(&mut info)?;
        let mut other = [0; 1];
        rdr.read_exact(&mut other)?;
        let shndx = rdr.read_u16::<T>()?;
        let value = rdr.read_u64::<T>()?;
        let size = rdr.read_u64::<T>()?;

        Ok(DynSymbol64 {
            st_name: name,
            st_value: value,
            st_size: size,
            st_info: info[0],
            st_other: other[0],
            st_shndx: shndx,
        })
    }
}



#[derive(Clone, Debug)]
pub struct Symtable {
    pub section_idx: u32,
    pub sec_name: String,
    pub entries: Vec<Symbol>,
}

/*Symbol table object to hold parsed symbol structs*/
impl Symtable {
    pub fn parse_sym_table<R,>(rdr: &mut R, sec_size: u64,
                               idx: u32, name: String,
                               data: EXEC::EI_DATA,
                               class: EXEC::EI_CLASS) -> Result<Symtable, std::io::Error>
        where R: Read {

        let mut symtab_t: Vec<Symbol> = Vec::new();
        let mut total_size = 0;

        loop {
            let size = match class {
                EXEC::EI_CLASS::ELFCLASS32 => {
                    match data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let sym = Symbol32::parse_symbol::<R, LittleEndian>(rdr)?;
                            symtab_t.push(Symbol::ThirtyTwo(sym));
                            std::mem::size_of::<Symbol32>()

                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let sym = Symbol32::parse_symbol::<R, BigEndian>(rdr)?;
                            symtab_t.push(Symbol::ThirtyTwo(sym));
                            std::mem::size_of::<Symbol32>()
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                },
                EXEC::EI_CLASS::ELFCLASS64 => {
                    match data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let sym = Symbol64::parse_symbol::<R, LittleEndian>(rdr)?;
                            symtab_t.push(Symbol::SixtyFour(sym));
                            std::mem::size_of::<Symbol64>()


                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let sym = Symbol64::parse_symbol::<R, BigEndian>(rdr)?;
                            symtab_t.push(Symbol::SixtyFour(sym));
                            std::mem::size_of::<Symbol64>()

                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                }
                _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                    "Elf not supported"))
            };

            let num_syms = sec_size / size as u64;

            total_size += 1;
            if total_size >= num_syms as usize {
                break;
            }
        }

        Ok(Symtable {
            section_idx: idx,
            sec_name: name,
            entries : symtab_t,
        })
    }

}



#[derive(Clone, Debug)]
pub struct DynSymtable {
    pub section_idx: u32,
    pub sec_name: String,
    pub entries: Vec<DynSymbol>,
}

/*Symbol table object to hold parsed symbol structs*/
impl DynSymtable {
    pub fn parse_dynsym_table<R,>(rdr: &mut R, sec_size: u64, idx: u32,
                                  name: String, data: EXEC::EI_DATA,
                                  class: EXEC::EI_CLASS) -> Result<DynSymtable, std::io::Error>
        where R: Read {
        let mut symtab_t: Vec<DynSymbol> = Vec::new();
        let mut total_size = 0;
        loop {
            let size = match class {
                EXEC::EI_CLASS::ELFCLASS32 => {
                    match data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let sym = DynSymbol32::parse_symbol::<R, LittleEndian>(rdr)?;
                            symtab_t.push(DynSymbol::ThirtyTwo(sym));
                            std::mem::size_of::<DynSymbol32>()
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let sym = DynSymbol32::parse_symbol::<R, BigEndian>(rdr)?;
                            symtab_t.push(DynSymbol::ThirtyTwo(sym));
                            std::mem::size_of::<DynSymbol32>()
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                },
                EXEC::EI_CLASS::ELFCLASS64 => {
                    match data {
                        EXEC::EI_DATA::ELFDATA2LSB => {
                            let sym = DynSymbol64::parse_symbol::<R, LittleEndian>(rdr)?;
                            symtab_t.push(DynSymbol::SixtyFour(sym));
                            std::mem::size_of::<DynSymbol64>()
                        }
                        EXEC::EI_DATA::ELFDATA2MSB => {
                            let sym = DynSymbol64::parse_symbol::<R, BigEndian>(rdr)?;
                            symtab_t.push(DynSymbol::SixtyFour(sym));
                            std::mem::size_of::<DynSymbol64>()
                        }
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                            "Elf not supported"))
                    }
                }
                _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                    "Elf not supported"))
            };
            let num_syms = sec_size / size as u64;
            //println!("dyn sym table contains {:?} entries", num_syms);
            total_size += 1;
            if total_size >= num_syms as usize {
                break;
            }
        }

        Ok(DynSymtable {
            section_idx: idx,
            sec_name: name,
            entries: symtab_t,
        })
    }

}

#[allow(non_camel_case_types)]
pub enum ST_bind{
    STB_LOCAL = 0,
    STB_GLOBAL = 1,
    STB_WEAK = 2,
    STB_LOPROC = 13,
    STB_HIPROC = 15,
}

#[allow(non_camel_case_types)]
pub enum ST_type{
    STT_NOTYPE = 0,
    STT_OBJECT = 1,
    STT_FUNC = 2,
    STT_SECTION = 3,
    STT_FILE = 4,
    STT_COMMON = 5,
    STT_TLS = 6,
    STT_LOPROC = 13,
    STT_HIPROC = 15,
}

#[allow(non_camel_case_types)]
pub enum SHN {
    SHN_UNDEF = 0,
    SHN_ABS,
    SHN_COMMON,
}

/*


#[allow(non_camel_case_types)]
enum St_info {
    STT_NOTYPE,
    STT_OBJECT,
    STT_FUNC,
    STT_SECTION,
    STT_FILE,
    STT_LOPROC,
    STT_HIPROC,
    STB_LOCAL,
    STB_GLOBAL,
    STB_WEAK,
    STB_LOPROC,
    STB_HIPROC,
}
#[allow(non_camel_case_types)]
enum St_other {
    STV_DEFAULT,
    STV_INTERNAL,
    STV_HIDDEN,
}

#define ELF_ST_BIND(x)		((x) >> 4)
#define ELF_ST_TYPE(x)		(((unsigned int) x) & 0xf)
#define ELF32_ST_BIND(x)	ELF_ST_BIND(x)
#define ELF32_ST_TYPE(x)	ELF_ST_TYPE(x)
#define ELF64_ST_BIND(x)	ELF_ST_BIND(x)
#define ELF64_ST_TYPE(x)	ELF_ST_TYPE(x)

typedef struct dynamic{
  Elf32_Sword d_tag;
  union{
    Elf32_Sword	d_val;
    Elf32_Addr	d_ptr;
  } d_un;
} Elf32_Dyn;

typedef struct {
  Elf64_Sxword d_tag;		/* entry tag value */
  union {
    Elf64_Xword d_val;
    Elf64_Addr d_ptr;
  } d_un;
} Elf64_Dyn;
*/