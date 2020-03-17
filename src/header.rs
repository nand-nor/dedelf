use std::fs::File;
use byteorder::*;
use std::io::{Read, Seek, SeekFrom};


/* Enum needed for various functions that support runtime parsing of ELF data*/
#[derive(Clone, Debug)]
pub enum ExecHeader {
    ThirtyTwo(ExecHeader32),
    SixtyFour(ExecHeader64)
}


#[derive(Clone, Debug)]
pub struct ExecHeader32 {
    pub e_ident: [u8; EXEC::EI_IDENT as usize],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[derive(Clone, Debug)]
pub struct ExecHeader64 {
    pub e_ident: [u8;  EXEC::EI_IDENT as usize],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ExecHeader32{

    pub fn new(ident_array: [u8; 16], etype: u16,
               emach: u16, evers: u32, eentry: u32,
               phoff: u32, shoff: u32, flags: u32, ehsize: u16,
               pehsize: u16, phnum: u16, shent: u16,
               shnum: u16, shstrndx: u16,) -> ExecHeader32{

        ExecHeader32{
            e_ident: ident_array,
            e_type: etype,
            e_machine : emach,
            e_version : evers,
            e_entry : eentry,
            e_phoff : phoff,
            e_shoff : shoff,
            e_flags : flags,
            e_ehsize : ehsize,
            e_phentsize : pehsize,
            e_phnum : phnum,
            e_shentsize : shent,
            e_shnum : shnum,
            e_shstrndx : shstrndx,
        }

    }

    pub fn parse_exec_header<R, B: ByteOrder>(file_ptr: &mut R)
                                    -> Result<ExecHeader32, std::io::Error>  where R: Read,{
        let mut ident_array = [0; EXEC::EI_IDENT];
        file_ptr.read_exact(&mut ident_array)?;
        let etype = file_ptr.read_u16::<B>()?;
        let emach = file_ptr.read_u16::<B>()?;
        let evers = file_ptr.read_u32::<B>()?;
        let eentry = file_ptr.read_u32::<B>()?;
        let phoff = file_ptr.read_u32::<B>()?;
        let shoff = file_ptr.read_u32::<B>()?;
        let flags = file_ptr.read_u32::<B>()?;
        let ehsize = file_ptr.read_u16::<B>()?;
        let pehsize = file_ptr.read_u16::<B>()?;
        let phnum = file_ptr.read_u16::<B>()?;
        let shent = file_ptr.read_u16::<B>()?;
        let shnum = file_ptr.read_u16::<B>()?;
        let shstrndx = file_ptr.read_u16::<B>()?;

        Ok(ExecHeader32{
            e_ident: ident_array,
            e_type: etype,
            e_machine : emach,
            e_version : evers,
            e_entry : eentry,
            e_phoff : phoff,
            e_shoff : shoff,
            e_flags : flags,
            e_ehsize : ehsize,
            e_phentsize : pehsize,
            e_phnum : phnum,
            e_shentsize : shent,
            e_shnum : shnum,
            e_shstrndx : shstrndx,
        })

    }

    /*TODO -- this can be improved; just write directly to file pointer rather than intermediate vec */
    pub fn write_header<B: ByteOrder>(&self, file_ptr: &mut File)-> Result<(),std::io::Error>{
        for &val in &self.e_ident{
            file_ptr.write_u8(val)?;
        }
        file_ptr.write_u16::<B>(self.e_type)?;
        file_ptr.write_u16::<B>(self.e_machine)?;
        file_ptr.write_u32::<B>(self.e_version)?;
        file_ptr.write_u32::<B>(self.e_entry)?;
        file_ptr.write_u32::<B>(self.e_phoff)?;
        file_ptr.write_u32::<B>(self.e_shoff)?;
        file_ptr.write_u32::<B>(self.e_flags)?;
        file_ptr.write_u16::<B>(self.e_ehsize)?;
        file_ptr.write_u16::<B>(self.e_phentsize)?;
        file_ptr.write_u16::<B>(self.e_phnum)?;
        file_ptr.write_u16::<B>(self.e_shentsize)?;
        file_ptr.write_u16::<B>(self.e_shnum)?;
        file_ptr.write_u16::<B>(self.e_shstrndx)?;
        Ok(())
    }

    /*
   * Note: The calling function is responsible for parsing a safe-to-downcast
   * value from u64 to either u8, u16, or u32
   */
    pub fn update_exec_header(&mut self, field: String,
                              val: u32, offset: Option<usize>)-> Result<(),std::io::Error>{
        match field.as_ref() {
            "e_ident" => {
                if let Some(offset) = offset {
                    self.e_ident[offset]=val as u8
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                        "Invalid exec header change option"))
                }
            },

            "e_type" =>  self.e_type = val as u16,
            "e_machine" => self.e_machine = val as u16,
            "e_version" => self.e_version =val,
            "e_entry" =>  self.e_entry = val,
            "e_phoff" => self.e_phoff = val,
            "e_shoff" => self.e_shoff =val,
            "e_flags" => self.e_flags=val,
            "e_ehsize" => self.e_ehsize=val as u16,
            "e_phentsize" => self.e_phentsize=val as u16,
            "e_phnum" => self.e_phnum=val as u16,
            "e_shentsize" => self.e_shentsize=val as u16,
            "e_shnum" => self.e_shnum=val as u16,
            "e_shstrndx" => self.e_shstrndx=val as u16,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                "Invalid exec header change option"))

        };
        Ok(())
    }



}
impl ExecHeader64{

    pub fn new(ident_array: [u8; EXEC::EI_IDENT], etype: u16,
               emach: u16, evers: u32, eentry: u64,
               phoff: u64, shoff: u64, flags: u32, ehsize: u16,
               pehsize: u16, phnum: u16, shent: u16,
               shnum: u16, shstrndx: u16,) -> ExecHeader64 {

        ExecHeader64{
            e_ident: ident_array,
            e_type: etype,
            e_machine : emach,
            e_version : evers,
            e_entry : eentry,
            e_phoff : phoff,
            e_shoff : shoff,
            e_flags : flags,
            e_ehsize : ehsize,
            e_phentsize : pehsize,
            e_phnum : phnum,
            e_shentsize : shent,
            e_shnum : shnum,
            e_shstrndx : shstrndx,
        }

    }

    pub fn parse_exec_header<R,B: ByteOrder>(file_ptr: &mut R,)
                            -> Result<ExecHeader64, std::io::Error> where R: Read,{
        let mut ident_array = [0; EXEC::EI_IDENT];
        file_ptr.read_exact(&mut ident_array)?;
        let etype = file_ptr.read_u16::<B>()?;
        let emach = file_ptr.read_u16::<B>()?;
        let evers = file_ptr.read_u32::<B>()?;
        let eentry = file_ptr.read_u64::<B>()?;
        let phoff = file_ptr.read_u64::<B>()?;
        let shoff = file_ptr.read_u64::<B>()?;
        let flags = file_ptr.read_u32::<B>()?;
        let ehsize = file_ptr.read_u16::<B>()?;
        let pehsize = file_ptr.read_u16::<B>()?;
        let phnum = file_ptr.read_u16::<B>()?;
        let shent = file_ptr.read_u16::<B>()?;
        let shnum = file_ptr.read_u16::<B>()?;
        let shstrndx = file_ptr.read_u16::<B>()?;

        Ok(ExecHeader64{
            e_ident: ident_array,
            e_type: etype,
            e_machine : emach,
            e_version : evers,
            e_entry : eentry,
            e_phoff : phoff,
            e_shoff : shoff,
            e_flags : flags,
            e_ehsize : ehsize,
            e_phentsize : pehsize,
            e_phnum : phnum,
            e_shentsize : shent,
            e_shnum : shnum,
            e_shstrndx : shstrndx,
        })

    }

    /* TODO: IMPROVE THIS, dont need the intermediate writer vec, just write directly to file pointer*/
    pub fn write_header<B: ByteOrder>(&self,
                                                 file_ptr: &mut File)->Result<(),std::io::Error>{
        file_ptr.seek( SeekFrom::Start(0))?;

        for &val in &self.e_ident{
            file_ptr.write_u8(val)?;
        }

        file_ptr.write_u16::<B>(self.e_type)?;
        file_ptr.write_u16::<B>(self.e_machine)?;
        file_ptr.write_u32::<B>(self.e_version)?;
        file_ptr.write_u64::<B>(self.e_entry)?;
        file_ptr.write_u64::<B>(self.e_phoff)?;
        file_ptr.write_u64::<B>(self.e_shoff)?;
        file_ptr.write_u32::<B>(self.e_flags)?;
        file_ptr.write_u16::<B>(self.e_ehsize)?;
        file_ptr.write_u16::<B>(self.e_phentsize)?;
        file_ptr.write_u16::<B>(self.e_phnum)?;
        file_ptr.write_u16::<B>(self.e_shentsize)?;
        file_ptr.write_u16::<B>(self.e_shnum)?;
        file_ptr.write_u16::<B>(self.e_shstrndx)?;
        Ok(())
    }


    /*
    * Note: The calling function is responsible for parsing a safe-to-downcast
    * value from u64 to either u8, u16, or u32
    */
    pub fn update_exec_header(&mut self, field: String,
                              val: u64, offset: Option<usize>)->Result<(),std::io::Error> {
        match field.as_ref() {
            "e_ident" => {
                if let Some(offset) = offset {
                    self.e_ident[offset]=val as u8
                } else{
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                        "Invalid exec header change option"))
                }
            },

            "e_type" =>  self.e_type = val as u16,
            "e_machine" => self.e_machine = val as u16,
            "e_version" => self.e_version =val as u32,
            "e_entry" =>  self.e_entry = val,
            "e_phoff" => self.e_phoff = val,
            "e_shoff" => self.e_shoff =val,
            "e_flags" => self.e_flags=val as u32,
            "e_ehsize" => self.e_ehsize=val as u16,
            "e_phentsize" => self.e_phentsize=val as u16,
            "e_phnum" => self.e_phnum=val as u16,
            "e_shentsize" => self.e_shentsize=val as u16,
            "e_shnum" => self.e_shnum=val as u16,
            "e_shstrndx" => self.e_shstrndx=val as u16,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                "Invalid exec header change option"))

        };
        Ok(())
    }

}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct ExecutiveHeader {
    pub data: EXEC::EI_DATA,
    pub class: EXEC::EI_CLASS,
    pub EH: ExecHeader,
}

impl ExecutiveHeader {
    pub fn new<R>(file_ptr: &mut R) -> Result<ExecutiveHeader,std::io::Error>
        where R: Read + Seek, {
        file_ptr.seek(SeekFrom::Start(0))?;

        let mut ident_array = [0; EXEC::EI_IDENT];
        file_ptr.read_exact(&mut ident_array)?;
        file_ptr.seek(SeekFrom::Start(0))?;

        let class: EXEC::EI_CLASS = match_class( ident_array[EXEC::_EI_CLASS]);
        let data: EXEC::EI_DATA = match_data(ident_array[EXEC::_EI_DATA]);

        let eh_t = match class {
            EXEC::EI_CLASS::ELFCLASSNONE => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "Elf class not supported"))
            },
            EXEC::EI_CLASS::ELFCLASS32=> {
                match data {
                    EXEC::EI_DATA::ELFDATANONE => {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                       "Elf data not supported"))
                    },
                    EXEC::EI_DATA::ELFDATA2LSB => {
                        let exec: ExecHeader32=
                            ExecHeader32::parse_exec_header::<R, LittleEndian>(file_ptr)?;
                        ExecHeader::ThirtyTwo(exec)

                    },
                    EXEC::EI_DATA::ELFDATA2MSB => {
                        let exec: ExecHeader32 =
                            ExecHeader32::parse_exec_header::<R, BigEndian>(file_ptr)?;
                        ExecHeader::ThirtyTwo(exec)
                    },
                    EXEC::EI_DATA::ELFDATAOTHER(_)=> {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                       "Elf data not supported"))

                    }
                }

            },
            EXEC::EI_CLASS::ELFCLASS64=>{
                match data {
                    EXEC::EI_DATA::ELFDATANONE => {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                       "Elf data not supported"))
                    },
                    EXEC::EI_DATA::ELFDATA2LSB => {
                        let exec: ExecHeader64=
                            ExecHeader64::parse_exec_header::<R, LittleEndian>(file_ptr)?;
                        ExecHeader::SixtyFour(exec)

                    },
                    EXEC::EI_DATA::ELFDATA2MSB => {
                        let exec: ExecHeader64 =
                            ExecHeader64::parse_exec_header::<R, BigEndian>(file_ptr)?;
                        ExecHeader::SixtyFour(exec)
                    },
                    EXEC::EI_DATA::ELFDATAOTHER(_)=> {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                       "Elf data not supported"))
                    }
                }
            },
            EXEC::EI_CLASS::ELFCLASSOTHER(_) =>{
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               "Elf class not supported"))
            },
        };

        Ok(ExecutiveHeader {
            class: class,
            data: data,
            EH: eh_t,
        })
    }


/*
    pub fn write_header(&self, file_ptr: &mut File ) -> Result<(),std::io::Error>{
        file_ptr.seek(SeekFrom::Start(0))?;
        self.write_exec_header(file_ptr)?;
        Ok(())
    }*/

    pub fn write_exec_header(&self, file_ptr: &mut File) -> Result<(),std::io::Error>{
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                match self.data {
                    EXEC::EI_DATA::ELFDATA2LSB => {
                        exec32.write_header::<LittleEndian>(file_ptr)
                    },
                    EXEC::EI_DATA::ELFDATA2MSB => {
                        exec32.write_header::<BigEndian>(file_ptr)
                    }
                    _ => {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                       "Elf class not supported"))
                    }
                }
            },
            ExecHeader::SixtyFour(exec64)=>{
                match self.data {
                    EXEC::EI_DATA::ELFDATA2LSB => {
                        exec64.write_header::<LittleEndian>(file_ptr)
                    },
                    EXEC::EI_DATA::ELFDATA2MSB => {
                        exec64.write_header::<BigEndian>(file_ptr)
                    }
                    _ => {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                       "Elf class not supported"))
                    }
                }
            },

        }
    }

    pub fn update_sht_offset(&mut self, by_size: u64){
        match &mut self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
               exec32.e_shoff += by_size as u32
            },
            ExecHeader::SixtyFour(exec64)=>{
                exec64.e_shoff += by_size
            },
        }
    }

    pub fn sht_offset(&self)-> SHTOffset {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                SHTOffset::ThirtyTwo(exec32.e_shoff)
            },
            ExecHeader::SixtyFour(exec64)=>{
                SHTOffset::SixtyFour(exec64.e_shoff)
            },
        }
    }

    pub fn pht_offset(&self)-> PHTOffset {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                PHTOffset::ThirtyTwo(exec32.e_phoff)
            },
            ExecHeader::SixtyFour(exec64)=>{
                PHTOffset::SixtyFour(exec64.e_phoff)
            },
        }
    }

    pub fn entry(&self)-> Entry {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                Entry::ThirtyTwo(exec32.e_entry)
            },
            ExecHeader::SixtyFour(exec64)=>{
                Entry::SixtyFour(exec64.e_entry)
            },
        }
    }

    pub fn ph_entry_num(&self)-> u16 {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
               exec32.e_phnum
            },
            ExecHeader::SixtyFour(exec64)=>{
                exec64.e_phnum
            },
        }
    }
    pub fn sh_entry_num(&self)-> u16 {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                exec32.e_shnum
            },
            ExecHeader::SixtyFour(exec64)=>{
                exec64.e_shnum
            },
        }
    }

    pub fn sh_entry_size(&self)-> u16 {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                exec32.e_shentsize
            },
            ExecHeader::SixtyFour(exec64)=>{
                exec64.e_shentsize
            },
        }
    }
    pub fn ph_entry_size(&self)-> u16 {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                exec32.e_phentsize
            },
            ExecHeader::SixtyFour(exec64)=>{
                exec64.e_phentsize
            },
        }
    }

    pub fn shstrndx(&self)-> u16 {
        match &self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                exec32.e_shstrndx
            },
            ExecHeader::SixtyFour(exec64)=>{
                exec64.e_shstrndx
            },
        }
    }


    //This function should only ever be called after obtaining 'safe' values for val i.e.
    //should be parsed to return a u8 or u16 appropriately so that we already know
    //up or downcasting is safe
    pub fn update_exec_header(&mut self, field: String, val: u64, offset: Option<usize> )
        -> Result<(),std::io::Error>{
        match &mut self.EH {
            ExecHeader::ThirtyTwo(exec32) => {
                exec32.update_exec_header(field, val as u32, offset)?;
            },
            ExecHeader::SixtyFour(exec64) => {
                exec64.update_exec_header(field, val, offset)?;
            },
        };
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Entry{
    ThirtyTwo(u32),
    SixtyFour(u64),
}

#[derive(Clone, Debug)]
pub enum SHTOffset{
    ThirtyTwo(u32),
    SixtyFour(u64),
}

#[derive(Clone, Debug)]
pub enum PHTOffset{
    ThirtyTwo(u32),
    SixtyFour(u64),
}

pub fn match_data(data: u8) -> EXEC::EI_DATA {
    match data {
        0  => EXEC::EI_DATA::ELFDATANONE,
        1 => EXEC::EI_DATA::ELFDATA2LSB,
        2 => EXEC::EI_DATA::ELFDATA2MSB,
        d => EXEC::EI_DATA::ELFDATAOTHER(d),
    }
}

pub fn match_class(class: u8) -> EXEC::EI_CLASS {
    match class {
        0 => EXEC::EI_CLASS::ELFCLASSNONE,
        1 => EXEC::EI_CLASS::ELFCLASS32,
        2 => EXEC::EI_CLASS::ELFCLASS64,
        c => EXEC::EI_CLASS::ELFCLASSOTHER(c),
    }
}

pub fn match_osabi(osabi: u8) -> EXEC::EI_OSABI {
    match osabi {
        0 => EXEC::EI_OSABI::ELFOSABI_NONE,
        1 => EXEC::EI_OSABI::ELFOSABI_HPUX,
        2 => EXEC::EI_OSABI::ELFOSABI_NETBSD,
        3 => EXEC::EI_OSABI::ELFOSABI_GNU,
        6 => EXEC::EI_OSABI::ELFOSABI_SOLARIS,
        7 => EXEC::EI_OSABI::ELFOSABI_AIX,
        8 => EXEC::EI_OSABI::ELFOSABI_IRIX,
        9 => EXEC::EI_OSABI::ELFOSABI_FREEBSD,
        10 => EXEC::EI_OSABI::ELFOSABI_TRU64,
        11 => EXEC::EI_OSABI::ELFOSABI_MODESTO,
        12 => EXEC::EI_OSABI::ELFOSABI_OPENBSD,
        64 => EXEC::EI_OSABI::ELFOSABI_ARM_AEABI,
        97 => EXEC::EI_OSABI::ELFOSABI_ARM,
        255 => EXEC::EI_OSABI::ELFOSABI_STANDALONE,
        osabi => EXEC::EI_OSABI::ELFOSABI_OTHER(osabi),
    }
}


pub fn match_data_as_str(data: String)-> Result<u8, std::io::Error>{
    match data.as_str(){
        "ELFDATANONE" =>Ok(0), //should this be supported?
        "ELFDATA2LSB" =>Ok(1),
        "ELFDATA2MSB" =>Ok(2),
        //ELFCLASSOTHER, //not supported
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Elf data not supported"))
    }
}

pub fn match_class_as_str(class: String)-> Result<u8, std::io::Error>{
    match class.as_str(){
        "ELFCLASSNONE" =>Ok(0), //should this be supported?
        "ELFCLASS32" =>Ok(1),
        "ELFCLASS64" =>Ok(2),
        //ELFCLASSOTHER, //not supported
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Elf class not supported"))
    }
}

/* comments from elf.h*/
pub fn match_osabi_as_str(osabi: String)-> Result<u8, std::io::Error>{
    match osabi.as_str(){
        "ELFOSABI_NONE" =>Ok(0),                    /* UNIX System V ABI */
        "ELFOSABI_SYSV" => Ok(0),                   /* Alias for  ELFOSABI_NONE */
        "ELFOSABI_HPUX" =>Ok(1),                    /* HP-UX */
        "ELFOSABI_NETBSD" =>Ok(2),                  /* NetBSD.  */
        "ELFOSABI_GNU" =>Ok(3),                     /* Object uses GNU ELF extensions.  */
        "ELFOSABI_LINUX" => Ok(3),                  /* Compatibility alias for ELFOSABI_GNU  */
        "ELFOSABI_SOLARIS" =>Ok(6),                 /* Sun Solaris.  */
        "ELFOSABI_AIX" =>Ok(7),                     /* IBM AIX.  */
        "ELFOSABI_IRIX" =>Ok(8),                    /* SGI Irix.  */
        "ELFOSABI_FREEBSD" =>Ok(9),                 /* FreeBSD.  */
        "ELFOSABI_TRU64" =>Ok(10),                  /* Compaq TRU64 UNIX.  */
        "ELFOSABI_MODESTO" =>Ok(11),                /* Novell Modesto.  */
        "ELFOSABI_OPENBSD" =>Ok( 12),               /* OpenBSD.  */
        "ELFOSABI_ARM_AEABI" =>Ok(64),              /* ARM EABI */
        "ELFOSABI_ARM" =>Ok(97),                    /* ARM */
        //ELFOSABI_OTHER(u8),                       //TODO should support 'other'?
        "ELFOSABI_STANDALONE" =>Ok(255),            /* Standalone (embedded) application */
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Elf osabi not supported"))

    }
}

pub fn match_version(e_vers: u32)-> Result<EXEC::EI_VERS, std::io::Error> {
    match e_vers{
        0 => Ok(EXEC::EI_VERS::EV_NONE),
        1 => Ok(EXEC::EI_VERS::EV_CURRENT),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            "Elf version not supported"))
    }
}
pub fn match_version_as_str(e_vers: String)-> Result<u32, std::io::Error> {
    match e_vers.as_str(){
        "EV_NONE"=> Ok(0),
        "EV_CURRENT"=>Ok(1),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            "Elf version not supported"))
    }
}


pub fn match_type(etype: u16)-> Result<EXEC::EI_TYPE, std::io::Error> {
        match etype {
            0 => Ok(EXEC::EI_TYPE::ET_NONE),
            1 => Ok(EXEC::EI_TYPE::ET_REL),
            2=> Ok(EXEC::EI_TYPE::ET_EXEC),
            3=> Ok(EXEC::EI_TYPE::ET_DYN),
            4=> Ok(EXEC::EI_TYPE::ET_CORE),
            0xfe00 => Ok(EXEC::EI_TYPE::ET_LOOS),
            0xfeff=> Ok(EXEC::EI_TYPE::ET_HIOS),
            0xff00=> Ok(EXEC::EI_TYPE::ET_LOPROC),
            0xffff=> Ok(EXEC::EI_TYPE::ET_HIPROC),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                "Elf type not supported"))
        }
}

pub fn match_type_as_str(etype: String)->Result<u16, std::io::Error>{
    match etype.as_str() {
        "ET_NONE"=> Ok(0),
        "ET_REL"=> Ok(1),
        "ET_EXEC"=> Ok(2),
        "ET_DYN"=> Ok(3),
        "ET_CORE"=> Ok(4),
        "ET_LOOS"=> Ok(0xfe00),
        "ET_HIOS"=> Ok(0xfeff),
        "ET_LOPROC"=> Ok(0xff00),
        "ET_HIPROC"=> Ok(0xffff),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            "Elf type not supported"))
    }
}

pub fn match_mach_as_str(mach: String) -> Result<u16, std::io::Error> {
    match mach.as_str(){
        "EM_NONE" => Ok(0),
        "EM_M32" => Ok(1),          /*	AT&T WE 32100 */
        "EM_SPARC" => Ok(2),        /*SPARC*/
        "EM_386" => Ok(3),          /*Intel 80386*/
        "EM_68K" => Ok(4),          /*Motorola 68000*/
        "EM_88K" => Ok(5),          /*	Motorola 88000*/
        //   RESERVED = 6,        /* 	Reserved for future use*/
        "EM_860" => Ok(7),          /*Intel 80860*/
        "EM_MIPS" => Ok(8),         /*	MIPS I Architecture */
        "EM_S370" => Ok(9),         /* 	IBM System/370 Processor */
        "EM_MIPS_RS3_LE" => Ok(10), /*	MIPS RS3000 Little-endian */
        //RESERVED 	11-14 //	/*Reserved for future use */
        "EM_PARISC" => Ok(15), /*Hewlett-Packard PA-RISC */
        //RESERVED  16 //	/*Reserved for future use */
        "EM_VPP500" => Ok(17),      /*Fujitsu VPP500 */
        "EM_SPARC32PLUS" => Ok(18), /*Enhanced instruction set SPARC */
        "EM_960" => Ok(19),         /*	Intel 80960 */
        "EM_PPC" => Ok(20),         /*	PowerPC */
        "EM_PPC64" => Ok(21),       /*	64-bit PowerPC */
        //RESERVED 	22-35 //	/*Reserved for future use */
        "EM_V800" => Ok(36),     /*	NEC V800 */
        "EM_FR20" => Ok(37),     /*	Fujitsu FR20 */
        "EM_RH32" => Ok(38),     /*TRW RH-32 */
        "EM_RCE" => Ok(39),      /*Motorola RCE */
        "EM_ARM" => Ok(40),      /*Advanced RISC Machines ARM */
        "EM_ALPHA" => Ok(41),    /*	Digital Alpha */
        "EM_SH" => Ok(42),       /*Hitachi SH */
        "EM_SPARCV9" => Ok(43),  /* 	SPARC Version 9 */
        "EM_TRICORE" => Ok(44),  /*	Siemens Tricore embedded processor */
        "EM_ARC" => Ok(45),      /*Argonaut RISC Core, Argonaut Technologies Inc. */
        "EM_H8_300" => Ok(46),   /*	Hitachi H8/300 */
        "EM_H8_300H" => Ok(47),  /*	Hitachi H8/300H */
        "EM_H8S" => Ok(48),      /*Hitachi H8S */
        "EM_H8_500" => Ok(49),   /*Hitachi H8/500 */
        "EM_IA_64" => Ok(50),    /*Intel IA-64 processor architecture */
        "EM_MIPS_X" => Ok(51),   /*Stanford MIPS-X */
        "EM_COLDFIRE" => Ok(52), /*	Motorola ColdFire */
        "EM_68HC12" => Ok(53),   /*Motorola M68HC12 */
        "EM_MMA" => Ok(54),      /*Fujitsu MMA Multimedia Accelerator */
        "EM_PCP" => Ok(55),      /*Siemens PCP */
        "EM_NCPU" => Ok(56),     /*Sony nCPU embedded RISC processor */
        "EM_NDR1" => Ok(57),     /*Denso NDR1 microprocessor */
        "EM_STARCORE" => Ok(58), /* 	Motorola Star*Core processor */
        "EM_ME16" => Ok(59),     /*Toyota ME16 processor */
        "EM_ST100" => Ok(60),    /*STMicroelectronics ST100 processor */
        "EM_TINYJ" => Ok(61),    /*Advanced Logic Corp. TinyJ embedded processor family */
        //Reserved 	62-65 	/*Reserved for future use */
        "EM_FX66" => Ok(66),     /*Siemens FX66 microcontroller */
        "EM_ST9PLUS" => Ok(67),  /*	STMicroelectronics ST9+ 8/16 bit microcontroller */
        "EM_ST7" => Ok(68),      /*STMicroelectronics ST7 8-bit microcontroller */
        "EM_68HC16" => Ok(69),   /*	Motorola MC68HC16 Microcontroller */
        "EM_68HC11" => Ok(70),   /*	Motorola MC68HC11 Microcontroller */
        "EM_68HC08" => Ok(71),   /*	Motorola MC68HC08 Microcontroller */
        "EM_68HC05" => Ok(72),   /*	Motorola MC68HC05 Microcontroller */
        "EM_SVX" => Ok(73),      /*Silicon Graphics SVx */
        "EM_ST19" => Ok(74),     /*	STMicroelectronics ST19 8-bit microcontroller */
        "EM_VAX" => Ok(75),      /*	Digital VAX */
        "EM_CRIS" => Ok(76),     /*	Axis Communications 32-bit embedded processor */
        "EM_JAVELIN" => Ok(77),  /*Infineon Technologies 32-bit embedded processor */
        "EM_FIREPATH" => Ok(78), /* 	Element 14 64-bit DSP Processor */
        "EM_ZSP" => Ok(79),      /*LSI Logic 16-bit DSP Processor */
        "EM_MMIX" => Ok(80),     /* 	Donald Knuth's educational 64-bit processor */
        "EM_HUANY" => Ok(81),    /*	Harvard University machine-independent object files */
        "EM_PRISM" => Ok(82),    /*	SiTera Prism */
        "EM_AVR" => Ok(83),           /* Atmel AVR 8-bit microcontroller */
        "EM_FR30" => Ok(84),          /* Fujitsu FR30 */
        "EM_D10V" => Ok(85),          /* Mitsubishi D10V */
        "EM_D30V" => Ok(86),          /* Mitsubishi D30V */
        "EM_V850" => Ok(87),          /* NEC v850 */
        "EM_M32R" => Ok(88),          /* Mitsubishi M32R */
        "EM_MN10300" => Ok(89),       /* Matsushita MN10300 */
        "EM_MN10200" => Ok(90),       /* Matsushita MN10200 */
        "EM_PJ" => Ok(91),            /* picoJava */
        "EM_OPENRISC" => Ok(92),      /* OpenRISC 32-bit embedded processor */
        "EM_ARC_COMPACT" => Ok(93),   /* ARC International ARCompact */
        "EM_XTENSA" => Ok(94),        /* Tensilica Xtensa Architecture */
        "EM_VIDEOCORE" => Ok(95),     /* Alphamosaic VideoCore */
        "EM_TMM_GPP" => Ok(96),       /* Thompson Multimedia General Purpose Proc */
        "EM_NS32K" => Ok(97),         /* National Semi. 32000 */
        "EM_TPC" => Ok(98),           /* Tenor Network TPC */
        "EM_SNP1K" => Ok(99),         /* Trebia SNP 1000 */
        "EM_ST200" => Ok(100),        /* STMicroelectronics ST200 */
        "EM_IP2K" => Ok(101),         /* Ubicom IP2xxx */
        "EM_MAX" => Ok(102),          /* MAX processor */
        "EM_CR" => Ok(103),           /* National Semi. CompactRISC */
        "EM_F2MC16" => Ok(104),       /* Fujitsu F2MC16 */
        "EM_MSP430" => Ok(105),       /* Texas Instruments msp430 */
        "EM_BLACKFIN" => Ok(106),     /* Analog Devices Blackfin DSP */
        "EM_SE_C33" => Ok(107),       /* Seiko Epson S1C33 family */
        "EM_SEP" => Ok(108),          /* Sharp embedded microprocessor */
        "EM_ARCA" => Ok(109),         /* Arca RISC */
        "EM_UNICORE" => Ok(110),      /* PKU-Unity & MPRC Peking Uni. mc series */
        "EM_EXCESS" => Ok(111),       /* eXcess configurable cpu */
        "EM_DXP" => Ok(112),          /* Icera Semi. Deep Execution Processor */
        "EM_ALTERA_NIOS2" => Ok(113), /* Altera Nios II */
        "EM_CRX" => Ok(114),          /* National Semi. CompactRISC CRX */
        "EM_XGATE" => Ok(115),        /* Motorola XGATE */
        "EM_C166" => Ok(116),         /* Infineon C16x/XC16x */
        "EM_M16C" => Ok(117),         /* Renesas M16C */
        "EM_DSPIC30F" => Ok(118),     /* Microchip Technology dsPIC30F */
        "EM_CE" => Ok(119),           /* Freescale Communication Engine RISC */
        "EM_M32C" => Ok(120),         /* Renesas M32C */
        /* reserved 121-130 */
        "EM_TSK3000" => Ok(131),       /* Altium TSK3000 */
        "EM_RS08" => Ok(132),          /* Freescale RS08 */
        "EM_SHARC" => Ok(133),         /* Analog Devices SHARC family */
        "EM_ECOG2" => Ok(134),         /* Cyan Technology eCOG2 */
        "EM_SCORE7" => Ok(135),        /* Sunplus S+core7 RISC */
        "EM_DSP24" => Ok(136),         /* New Japan Radio (NJR) 24-bit DSP */
        "EM_VIDEOCORE3" => Ok(137),    /* Broadcom VideoCore III */
        "EM_LATTICEMICO32" => Ok(138), /* RISC for Lattice FPGA */
        "EM_SE_C17" => Ok(139),        /* Seiko Epson C17 */
        "EM_TI_C6000" => Ok(140),      /* Texas Instruments TMS320C6000 DSP */
        "EM_TI_C2000" => Ok(141),      /* Texas Instruments TMS320C2000 DSP */
        "EM_TI_C5500" => Ok(142),      /* Texas Instruments TMS320C55x DSP */
        "EM_TI_ARP32" => Ok(143),      /* Texas Instruments App. Specific RISC */
        "EM_TI_PRU" => Ok(144),        /* Texas Instruments Prog. Realtime Unit */
        /* reserved 145-159 */
        "EM_MMDSP_PLUS" => Ok(160),  /* STMicroelectronics 64bit VLIW DSP */
        "EM_CYPRESS_M8C" => Ok(161), /* Cypress M8C */
        "EM_R32C" => Ok(162),        /* Renesas R32C */
        "EM_TRIMEDIA" => Ok(163),    /* NXP Semi. TriMedia */
        "EM_QDSP6" => Ok(164),       /* QUALCOMM DSP6 */
        "EM_8051" => Ok(165),        /* Intel 8051 and variants */
        "EM_STXP7X" => Ok(166),      /* STMicroelectronics STxP7x */
        "EM_NDS32" => Ok(167),       /* Andes Tech. compact code emb. RISC */
        "EM_ECOG1X" => Ok(168),      /* Cyan Technology eCOG1X */
        "EM_MAXQ30" => Ok(169),      /* Dallas Semi. MAXQ30 mc */
        "EM_XIMO16" => Ok(170),      /* New Japan Radio (NJR) 16-bit DSP */
        "EM_MANIK" => Ok(171),       /* M2000 Reconfigurable RISC */
        "EM_CRAYNV2" => Ok(172),     /* Cray NV2 vector architecture */
        "EM_RX" => Ok(173),          /* Renesas RX */
        "EM_METAG" => Ok(174),       /* Imagination Tech. META */
        "EM_MCST_ELBRUS" => Ok(175), /* MCST Elbrus */
        "EM_ECOG16" => Ok(176),      /* Cyan Technology eCOG16 */
        "EM_CR16" => Ok(177),        /* National Semi. CompactRISC CR16 */
        "EM_ETPU" => Ok(178),        /* Freescale Extended Time Processing Unit */
        "EM_SLE9X" => Ok(179),       /* Infineon Tech. SLE9X */
        "EM_L10M" => Ok(180),        /* Intel L10M */
        "EM_K10M" => Ok(181),        /* Intel K10M */
        /* reserved 182 */
        "EM_AARCH64" => Ok(183), /* ARM AARCH64 */
        /* reserved 184 */
        "EM_AVR32" => Ok(185),        /* Amtel 32-bit microprocessor */
        "EM_STM8" => Ok(186),         /* STMicroelectronics STM8 */
        "EM_TILE64" => Ok(187),       /* Tileta TILE64 */
        "EM_TILEPRO" => Ok(188),      /* Tilera TILEPro */
        "EM_MICROBLAZE" => Ok(189),   /* Xilinx MicroBlaze */
        "EM_CUDA" => Ok(190),         /* NVIDIA CUDA */
        "EM_TILEGX" => Ok(191),       /* Tilera TILE-Gx */
        "EM_CLOUDSHIELD" => Ok(192),  /* CloudShield */
        "EM_COREA_1ST" => Ok(193),    /* KIPO-KAIST Core-A 1st gen. */
        "EM_COREA_2ND" => Ok(194),    /* KIPO-KAIST Core-A 2nd gen. */
        "EM_ARC_COMPACT2" => Ok(195), /* Synopsys ARCompact V2 */
        "EM_OPEN8" => Ok(196),        /* Open8 RISC */
        "EM_RL78" => Ok(197),         /* Renesas RL78 */
        "EM_VIDEOCORE5" => Ok(198),   /* Broadcom VideoCore V */
        "EM_78KOR" => Ok(199),        /* Renesas 78KOR */
        "EM_56800EX" => Ok(200),      /* Freescale 56800EX DSC */
        "EM_BA1" => Ok(201),          /* Beyond BA1 */
        "EM_BA2" => Ok(202),          /* Beyond BA2 */
        "EM_XCORE" => Ok(203),        /* XMOS xCORE */
        "EM_MCHP_PIC" => Ok(204),     /* Microchip 8-bit PIC(r) */
        /* reserved 205-209 */
        "EM_KM32" => Ok(210),        /* KM211 KM32 */
        "EM_KMX32" => Ok(211),       /* KM211 KMX32 */
        "EM_EMX16" => Ok(212),       /* KM211 KMX16 */
        "EM_EMX8" => Ok(213),        /* KM211 KMX8 */
        "EM_KVARC" => Ok(214),       /* KM211 KVARC */
        "EM_CDP" => Ok(215),         /* Paneve CDP */
        "EM_COGE" => Ok(216),        /* Cognitive Smart Memory Processor */
        "EM_COOL" => Ok(217),        /* Bluechip CoolEngine */
        "EM_NORC" => Ok(218),        /* Nanoradio Optimized RISC */
        "EM_CSR_KALIMBA" => Ok(219), /* CSR Kalimba */
        "EM_Z80" => Ok(220),         /* Zilog Z80 */
        "EM_VISIUM" => Ok(221),      /* Controls and Data Services VISIUMcore */
        "EM_FT32" => Ok(222),        /* FTDI Chip FT32 */
        "EM_MOXIE" => Ok(223),       /* Moxie processor */
        "EM_AMDGPU" => Ok(224),      /* AMD GPU */
        /* reserved 225-242 */
        "EM_RISCV" => Ok(243), /* RISC-V */
        "EM_BPF" => Ok(247),   /* Linux BPF -- in-kernel virtual machine */
        "EM_CSKY" => Ok(252),  /* C-SKY */
        "EM_NUM" => Ok(253),
        /* Old spellings/synonyms.  */
          "EM_ARC_A5"    =>    Ok(93),
        /* If it is necessary to assign new unofficial EM_* values, please
           pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
           chances of collision with official or non-GNU unofficial values.  */
        //EM_ALPHA     =   0x9026,
    _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Elf mach not supported"))

    }
}

//TODO check that all machine options are represented
/*comments from elf.h*/
pub fn match_mach(mach: u16) -> Result<EXEC::EI_MACH, std::io::Error> {
    match mach {
        0=> Ok(EXEC::EI_MACH::EM_NONE ),
        1=> Ok(EXEC::EI_MACH::EM_M32 ),         //	AT&T WE 32100
        2=> Ok(EXEC::EI_MACH::EM_SPARC ),       //SPARC
        3=> Ok(EXEC::EI_MACH::EM_386 ),         //Intel 80386
        4=> Ok(EXEC::EI_MACH::EM_68K ),         //Motorola 68000
        5=> Ok(EXEC::EI_MACH::EM_88K ),         //	Motorola 88000
        // RESERVED  6          // 	Reserved for future use
        7=> Ok(EXEC::EI_MACH::EM_860 ),          //Intel 80860
        8=> Ok(EXEC::EI_MACH::EM_MIPS ),        //	MIPS I Architecture
        9=> Ok(EXEC::EI_MACH::EM_S370 ),        // 	IBM System/370 Processor
        10=> Ok(EXEC::EI_MACH::EM_MIPS_RS3_LE ), //	MIPS RS3000 Little-endian
        //RESERVED 	11-14 //	Reserved for future use
        15=> Ok(EXEC::EI_MACH::EM_PARISC ), //Hewlett-Packard PA-RISC
        //RESERVED 	16 //	Reserved for future use
        17=> Ok(EXEC::EI_MACH::EM_VPP500 ),     //Fujitsu VPP500
        18=> Ok(EXEC::EI_MACH::EM_SPARC32PLUS ), //Enhanced instruction set SPARC
        19=> Ok(EXEC::EI_MACH::EM_960 ),        //	Intel 80960
        20=> Ok(EXEC::EI_MACH::EM_PPC ),         //	PowerPC
        21=> Ok(EXEC::EI_MACH::EM_PPC64 ),       //	64-bit PowerPC
        //RESERVED 	22-35 //	Reserved for future use
        36=> Ok(EXEC::EI_MACH::EM_V800 ),    //	NEC V800
        37=> Ok(EXEC::EI_MACH::EM_FR20 ),    //	Fujitsu FR20
        38=> Ok(EXEC::EI_MACH::EM_RH32 ),     //TRW RH-32
        39=> Ok(EXEC::EI_MACH::EM_RCE ),     //Motorola RCE
        40=> Ok(EXEC::EI_MACH::EM_ARM ),      //Advanced RISC Machines ARM
        41=> Ok(EXEC::EI_MACH::EM_ALPHA ),   //	Digital Alpha
        42=> Ok(EXEC::EI_MACH::EM_SH ),       //Hitachi SH
        43=> Ok(EXEC::EI_MACH::EM_SPARCV9 ),  // 	SPARC Version 9
        44=> Ok(EXEC::EI_MACH::EM_TRICORE ), //	Siemens Tricore embedded processor
        45=> Ok(EXEC::EI_MACH::EM_ARC),      //Argonaut RISC Core, Argonaut Technologies Inc.
        46=> Ok(EXEC::EI_MACH::EM_H8_300 ),   //	Hitachi H8/300
        47=> Ok(EXEC::EI_MACH::EM_H8_300H ),  //	Hitachi H8/300H
        48=> Ok(EXEC::EI_MACH::EM_H8S ),      //Hitachi H8S
        49=> Ok(EXEC::EI_MACH::EM_H8_500 ),   //Hitachi H8/500
        50=> Ok(EXEC::EI_MACH::EM_IA_64 ),   //Intel IA-64 processor architecture
        51=> Ok(EXEC::EI_MACH::EM_MIPS_X ),  //Stanford MIPS-X
        52=> Ok(EXEC::EI_MACH::EM_COLDFIRE ), //	Motorola ColdFire
        53=> Ok(EXEC::EI_MACH::EM_68HC12 ),   //Motorola M68HC12
        54=> Ok(EXEC::EI_MACH::EM_MMA),     //Fujitsu MMA Multimedia Accelerator
        55=> Ok(EXEC::EI_MACH::EM_PCP ),     //Siemens PCP
        56=> Ok(EXEC::EI_MACH::EM_NCPU ),     //Sony nCPU embedded RISC processor
        57=> Ok(EXEC::EI_MACH::EM_NDR1 ),    //Denso NDR1 microprocessor
        58=> Ok(EXEC::EI_MACH::EM_STARCORE ), // 	Motorola Star*Core processor
        59=> Ok(EXEC::EI_MACH::EM_ME16 ),    //Toyota ME16 processor
        60=> Ok(EXEC::EI_MACH::EM_ST100 ),    //STMicroelectronics ST100 processor
        61=> Ok(EXEC::EI_MACH::EM_TINYJ ),  //Advanced Logic Corp. TinyJ embedded processor family
        //Reserved 	62-65 	//Reserved for future use
        66=> Ok(EXEC::EI_MACH::EM_FX66 ),     //Siemens FX66 microcontroller
        67=> Ok(EXEC::EI_MACH::EM_ST9PLUS ),  //	STMicroelectronics ST9+ 8/16 bit microcontroller
        68=> Ok(EXEC::EI_MACH::EM_ST7 ),     //STMicroelectronics ST7 8-bit microcontroller
        69=> Ok(EXEC::EI_MACH::EM_68HC16 ),  //	Motorola MC68HC16 Microcontroller
        70=> Ok(EXEC::EI_MACH::EM_68HC11 ),   //	Motorola MC68HC11 Microcontroller
        71=> Ok(EXEC::EI_MACH::EM_68HC08 ),   //	Motorola MC68HC08 Microcontroller
        72=> Ok(EXEC::EI_MACH::EM_68HC05 ),  //	Motorola MC68HC05 Microcontroller
        73=> Ok(EXEC::EI_MACH::EM_SVX ),     //Silicon Graphics SVx
        74=> Ok(EXEC::EI_MACH::EM_ST19 ),    //	STMicroelectronics ST19 8-bit microcontroller
        75=> Ok(EXEC::EI_MACH::EM_VAX ),     //	Digital VAX
        76=> Ok(EXEC::EI_MACH::EM_CRIS ),     //	Axis Communications 32-bit embedded processor
        77=> Ok(EXEC::EI_MACH::EM_JAVELIN ),  //Infineon Technologies 32-bit embedded processor
        78=> Ok(EXEC::EI_MACH::EM_FIREPATH ), // 	Element 14 64-bit DSP Processor
        79=> Ok(EXEC::EI_MACH::EM_ZSP ),      //LSI Logic 16-bit DSP Processor
        80=> Ok(EXEC::EI_MACH::EM_MMIX ),     // 	Donald Knuth's educational 64-bit processor
        81=> Ok(EXEC::EI_MACH::EM_HUANY ),    //	Harvard University machine-independent object files
        82=> Ok(EXEC::EI_MACH::EM_PRISM ),    //	SiTera Prism
        83=> Ok(EXEC::EI_MACH::EM_AVR ),          /* Atmel AVR 8-bit microcontroller */
        84=> Ok(EXEC::EI_MACH::EM_FR30 ),          /* Fujitsu FR30 */
        85=> Ok(EXEC::EI_MACH::EM_D10V ),         /* Mitsubishi D10V */
        86=> Ok(EXEC::EI_MACH::EM_D30V ),          /* Mitsubishi D30V */
       87=> Ok(EXEC::EI_MACH::EM_V850 ),          /* NEC v850 */
        88=> Ok(EXEC::EI_MACH::EM_M32R),          /* Mitsubishi M32R */
        89=> Ok(EXEC::EI_MACH::EM_MN10300 ),      /* Matsushita MN10300 */
        90=> Ok(EXEC::EI_MACH::EM_MN10200 ),      /* Matsushita MN10200 */
        91=> Ok(EXEC::EI_MACH::EM_PJ ),            /* picoJava */
        92=> Ok(EXEC::EI_MACH::EM_OPENRISC ),      /* OpenRISC 32-bit embedded processor */
        93=> Ok(EXEC::EI_MACH::EM_ARC_COMPACT ),   /* ARC International ARCompact */
        94=> Ok(EXEC::EI_MACH::EM_XTENSA ),       /* Tensilica Xtensa Architecture */
        95=> Ok(EXEC::EI_MACH::EM_VIDEOCORE ),    /* Alphamosaic VideoCore */
        96=> Ok(EXEC::EI_MACH::EM_TMM_GPP ),      /* Thompson Multimedia General Purpose Proc */
        97=> Ok(EXEC::EI_MACH::EM_NS32K),         /* National Semi. 32000 */
        98=> Ok(EXEC::EI_MACH::EM_TPC ),           /* Tenor Network TPC */
        99=> Ok(EXEC::EI_MACH::EM_SNP1K ),         /* Trebia SNP 1000 */
        100=> Ok(EXEC::EI_MACH::EM_ST200 ),       /* STMicroelectronics ST200 */
        101=> Ok(EXEC::EI_MACH::EM_IP2K ),         /* Ubicom IP2xxx */
        102=> Ok(EXEC::EI_MACH::EM_MAX ),          /* MAX processor */
        103=> Ok(EXEC::EI_MACH::EM_CR ),          /* National Semi. CompactRISC */
        104=> Ok(EXEC::EI_MACH::EM_F2MC16 ),      /* Fujitsu F2MC16 */
        105=> Ok(EXEC::EI_MACH::EM_MSP430 ),       /* Texas Instruments msp430 */
        106=> Ok(EXEC::EI_MACH::EM_BLACKFIN ),     /* Analog Devices Blackfin DSP */
        107=> Ok(EXEC::EI_MACH::EM_SE_C33 ),       /* Seiko Epson S1C33 family */
        108=> Ok(EXEC::EI_MACH::EM_SEP ),         /* Sharp embedded microprocessor */
        109=> Ok(EXEC::EI_MACH::EM_ARCA ),         /* Arca RISC */
        110=> Ok(EXEC::EI_MACH::EM_UNICORE ),      /* PKU-Unity & MPRC Peking Uni. mc series */
        111=> Ok(EXEC::EI_MACH::EM_EXCESS ),       /* eXcess configurable cpu */
        112=> Ok(EXEC::EI_MACH::EM_DXP ),          /* Icera Semi. Deep Execution Processor */
       113 => Ok(EXEC::EI_MACH::EM_ALTERA_NIOS2 ),/* Altera Nios II */
        114=> Ok(EXEC::EI_MACH::EM_CRX ),          /* National Semi. CompactRISC CRX */
        115=> Ok(EXEC::EI_MACH::EM_XGATE ),       /* Motorola XGATE */
        116=> Ok(EXEC::EI_MACH::EM_C166 ),        /* Infineon C16x/XC16x */
        117=> Ok(EXEC::EI_MACH::EM_M16C ),         /* Renesas M16C */
        118=> Ok(EXEC::EI_MACH::EM_DSPIC30F ),    /* Microchip Technology dsPIC30F */
        119=> Ok(EXEC::EI_MACH::EM_CE ),          /* Freescale Communication Engine RISC */
        120=> Ok(EXEC::EI_MACH::EM_M32C ),         /* Renesas M32C */
        /* reserved 121-130 */
        131=> Ok(EXEC::EI_MACH::EM_TSK3000 ),      /* Altium TSK3000 */
        132=> Ok(EXEC::EI_MACH::EM_RS08 ),          /* Freescale RS08 */
        133=> Ok(EXEC::EI_MACH::EM_SHARC ),        /* Analog Devices SHARC family */
        134=> Ok(EXEC::EI_MACH::EM_ECOG2 ),         /* Cyan Technology eCOG2 */
        135=> Ok(EXEC::EI_MACH::EM_SCORE7 ),       /* Sunplus S+core7 RISC */
        136=> Ok(EXEC::EI_MACH::EM_DSP24 ),        /* New Japan Radio (NJR) 24-bit DSP */
        137=> Ok(EXEC::EI_MACH::EM_VIDEOCORE3 ),   /* Broadcom VideoCore III */
        138=> Ok(EXEC:: EI_MACH::EM_LATTICEMICO32 ), /* RISC for Lattice FPGA */
        139=> Ok(EXEC::EI_MACH::EM_SE_C17),        /* Seiko Epson C17 */
        140=> Ok(EXEC::EI_MACH::EM_TI_C6000 ),     /* Texas Instruments TMS320C6000 DSP */
        141=> Ok(EXEC::EI_MACH::EM_TI_C2000 ),      /* Texas Instruments TMS320C2000 DSP */
        142=> Ok(EXEC::EI_MACH::EM_TI_C5500 ),     /* Texas Instruments TMS320C55x DSP */
        143=> Ok(EXEC::EI_MACH::EM_TI_ARP32 ),      /* Texas Instruments App. Specific RISC */
        144=> Ok(EXEC::EI_MACH::EM_TI_PRU ),       /* Texas Instruments Prog. Realtime Unit */
        /* reserved 145-159 */
        160=> Ok(EXEC::EI_MACH::EM_MMDSP_PLUS ), /* STMicroelectronics 64bit VLIW DSP */
        161=> Ok(EXEC::EI_MACH::EM_CYPRESS_M8C ), /* Cypress M8C */
        162=> Ok(EXEC::EI_MACH::EM_R32C ),        /* Renesas R32C */
        163=> Ok(EXEC::EI_MACH::EM_TRIMEDIA ),    /* NXP Semi. TriMedia */
        164=> Ok(EXEC::EI_MACH::EM_QDSP6 ),      /* QUALCOMM DSP6 */
        165=> Ok(EXEC::EI_MACH::EM_8051 ),        /* Intel 8051 and variants */
        166=> Ok(EXEC::EI_MACH::EM_STXP7X ),      /* STMicroelectronics STxP7x */
        167=> Ok(EXEC::EI_MACH::EM_NDS32 ),       /* Andes Tech. compact code emb. RISC */
        168=> Ok(EXEC::EI_MACH::EM_ECOG1X ),      /* Cyan Technology eCOG1X */
        169=> Ok(EXEC::EI_MACH::EM_MAXQ30 ),     /* Dallas Semi. MAXQ30 mc */
        170=> Ok(EXEC::EI_MACH::EM_XIMO16 ),      /* New Japan Radio (NJR) 16-bit DSP */
        171=> Ok(EXEC::EI_MACH::EM_MANIK ),       /* M2000 Reconfigurable RISC */
        172=> Ok(EXEC::EI_MACH::EM_CRAYNV2 ),    /* Cray NV2 vector architecture */
        173=> Ok(EXEC::EI_MACH::EM_RX ),         /* Renesas RX */
        174=> Ok(EXEC::EI_MACH::EM_METAG ),       /* Imagination Tech. META */
        175=> Ok(EXEC::EI_MACH::EM_MCST_ELBRUS ), /* MCST Elbrus */
        176=> Ok(EXEC::EI_MACH::EM_ECOG16 ),     /* Cyan Technology eCOG16 */
        177=> Ok(EXEC::EI_MACH::EM_CR16 ),       /* National Semi. CompactRISC CR16 */
        178=> Ok(EXEC::EI_MACH::EM_ETPU ),       /* Freescale Extended Time Processing Unit */
        179=> Ok(EXEC::EI_MACH::EM_SLE9X ),       /* Infineon Tech. SLE9X */
        180=> Ok(EXEC::EI_MACH::EM_L10M ),        /* Intel L10M */
        181=> Ok(EXEC::EI_MACH::EM_K10M ),        /* Intel K10M */
        /* reserved 182 */
        183=> Ok(EXEC::EI_MACH::EM_AARCH64 ), /* ARM AARCH64 */
        /* reserved 184 */
        185=> Ok(EXEC::EI_MACH::EM_AVR32 ),        /* Amtel 32-bit microprocessor */
        186=> Ok(EXEC::EI_MACH::EM_STM8 ),         /* STMicroelectronics STM8 */
        187=> Ok(EXEC::EI_MACH::EM_TILE64),       /* Tileta TILE64 */
        188=> Ok(EXEC::EI_MACH::EM_TILEPRO ),     /* Tilera TILEPro */
        189=> Ok(EXEC::EI_MACH::EM_MICROBLAZE ),  /* Xilinx MicroBlaze */
        190=> Ok(EXEC::EI_MACH::EM_CUDA ),         /* NVIDIA CUDA */
        191=> Ok(EXEC::EI_MACH::EM_TILEGX ),       /* Tilera TILE-Gx */
        192=> Ok(EXEC::EI_MACH::EM_CLOUDSHIELD ),  /* CloudShield */
        193=> Ok(EXEC::EI_MACH::EM_COREA_1ST ),   /* KIPO-KAIST Core-A 1st gen. */
        194=> Ok(EXEC::EI_MACH::EM_COREA_2ND ),   /* KIPO-KAIST Core-A 2nd gen. */
        195=> Ok(EXEC::EI_MACH::EM_ARC_COMPACT2 ), /* Synopsys ARCompact V2 */
        196=> Ok(EXEC::EI_MACH::EM_OPEN8 ),       /* Open8 RISC */
        197=> Ok(EXEC::EI_MACH::EM_RL78 ),         /* Renesas RL78 */
        198=> Ok(EXEC::EI_MACH::EM_VIDEOCORE5 ),   /* Broadcom VideoCore V */
        199=> Ok(EXEC::EI_MACH::EM_78KOR ),       /* Renesas 78KOR */
        200=> Ok(EXEC::EI_MACH::EM_56800EX ),     /* Freescale 56800EX DSC */
        201=> Ok(EXEC::EI_MACH::EM_BA1),       /* Beyond BA1 */
        202=> Ok(EXEC::EI_MACH::EM_BA2),          /* Beyond BA2 */
        203=> Ok(EXEC::EI_MACH::EM_XCORE),       /* XMOS xCORE */
        204=> Ok(EXEC::EI_MACH::EM_MCHP_PIC),    /* Microchip 8-bit PIC(r) */
        /* reserved 205-209 */
        210=> Ok(EXEC::EI_MACH::EM_KM32),       /* KM211 KM32 */
        211=> Ok(EXEC::EI_MACH::EM_KMX32),       /* KM211 KMX32 */
        212=> Ok(EXEC::EI_MACH::EM_EMX16),       /* KM211 KMX16 */
        213=> Ok(EXEC::EI_MACH::EM_EMX8),       /* KM211 KMX8 */
        214=> Ok(EXEC::EI_MACH::EM_KVARC),      /* KM211 KVARC */
        215=> Ok(EXEC::EI_MACH::EM_CDP),        /* Paneve CDP */
        216=> Ok(EXEC::EI_MACH::EM_COGE),       /* Cognitive Smart Memory Processor */
        217=> Ok(EXEC::EI_MACH::EM_COOL),       /* Bluechip CoolEngine */
        218=> Ok(EXEC::EI_MACH::EM_NORC),       /* Nanoradio Optimized RISC */
        219=> Ok(EXEC::EI_MACH::EM_CSR_KALIMBA), /* CSR Kalimba */
        220=> Ok(EXEC::EI_MACH::EM_Z80),         /* Zilog Z80 */
        221=> Ok(EXEC::EI_MACH::EM_VISIUM),      /* Controls and Data Services VISIUMcore */
        222=> Ok(EXEC::EI_MACH::EM_FT32),        /* FTDI Chip FT32 */
        223=> Ok(EXEC::EI_MACH::EM_MOXIE),       /* Moxie processor */
        224=> Ok(EXEC::EI_MACH::EM_AMDGPU),     /* AMD GPU */
        /* reserved 225-242 */
        243=> Ok(EXEC::EI_MACH::EM_RISCV), /* RISC-V */
        /*244-246*/
        247=> Ok(EXEC::EI_MACH::EM_BPF),   /* Linux BPF -- in-kernel virtual machine */
        /*248-251*/
        252=> Ok(EXEC::EI_MACH::EM_CSKY),  /* C-SKY */
        253 => Ok(EXEC::EI_MACH::EM_NUM),
        /* Old spellings/synonyms.  */
        //  EM_ARC_A5    =    EM_ARC_COMPACT,
        /* If it is necessary to assign new unofficial EM_* values, please
           pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
           chances of collision with official or non-GNU unofficial values.  */
        //    EM_ALPHA     =>   0x9026,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Elf mach not supported"))

    }

}



#[allow(non_camel_case_types, non_snake_case)]
pub(super) mod EXEC {


    pub const _EI_MAG0: usize = 0;
    pub const _EI_MAG1: usize = 1;
    pub const _EI_MAG2: usize = 2;
    pub const _EI_MAG3: usize = 3;
    pub const _EI_CLASS: usize = 4;
    pub const _EI_DATA: usize = 5;
    pub const _EI_VERSION: usize = 6;
    pub const _EI_OSABI: usize = 7;
    pub const _EI_ABIVERSION: usize = 8;
    pub const _EI_PAD: usize = 9;
    pub const _EI_NIDENT: usize = 16;

    pub const EI_IDENT:usize = 16;

    #[repr(u16)]
    #[allow(non_camel_case_types)]
    pub enum EI_TYPE {
        ET_NONE = 0,
        ET_REL = 1,
        ET_EXEC = 2,
        ET_DYN = 3,
        ET_CORE = 4,
        ET_LOOS = 0xfe00,
        ET_HIOS = 0xfeff,
        ET_LOPROC = 0xff00,
        ET_HIPROC = 0xffff,
    }

    impl EI_TYPE {
        pub fn as_u16(&self) -> u16 {
            match self {
                EI_TYPE::ET_NONE => 0,
                EI_TYPE::ET_REL => 1,
                EI_TYPE::ET_EXEC => 2,
                EI_TYPE::ET_DYN => 3,
                EI_TYPE::ET_CORE => 4,
                EI_TYPE::ET_LOOS => 0xfe00,
                EI_TYPE::ET_HIOS => 0xfeff,
                EI_TYPE::ET_LOPROC => 0xff00,
                EI_TYPE::ET_HIPROC => 0xffff,
            }
        }
    }
    #[repr(u16)]
    #[allow(non_camel_case_types)]
    pub enum EI_MACH {
        EM_NONE = 0,
        EM_M32 = 1,          //	AT&T WE 32100
        EM_SPARC = 2,        //SPARC
        EM_386 = 3,          //Intel 80386
        EM_68K = 4,          //Motorola 68000
        EM_88K = 5,          //	Motorola 88000
     //   RESERVED = 6,        // 	Reserved for future use
        EM_860 = 7,          //Intel 80860
        EM_MIPS = 8,         //	MIPS I Architecture
        EM_S370 = 9,         // 	IBM System/370 Processor
        EM_MIPS_RS3_LE = 10, //	MIPS RS3000 Little-endian
        //RESERVED 	11-14 //	Reserved for future use
        EM_PARISC = 15, //Hewlett-Packard PA-RISC
        //RESERVED 	16 //	Reserved for future use
        EM_VPP500 = 17,      //Fujitsu VPP500
        EM_SPARC32PLUS = 18, //Enhanced instruction set SPARC
        EM_960 = 19,         //	Intel 80960
        EM_PPC = 20,         //	PowerPC
        EM_PPC64 = 21,       //	64-bit PowerPC
        //RESERVED 	22-35 //	Reserved for future use
        EM_V800 = 36,     //	NEC V800
        EM_FR20 = 37,     //	Fujitsu FR20
        EM_RH32 = 38,     //TRW RH-32
        EM_RCE = 39,      //Motorola RCE
        EM_ARM = 40,      //Advanced RISC Machines ARM
        EM_ALPHA = 41,    //	Digital Alpha
        EM_SH = 42,       //Hitachi SH
        EM_SPARCV9 = 43,  // 	SPARC Version 9
        EM_TRICORE = 44,  //	Siemens Tricore embedded processor
        EM_ARC = 45,      //Argonaut RISC Core, Argonaut Technologies Inc.
        EM_H8_300 = 46,   //	Hitachi H8/300
        EM_H8_300H = 47,  //	Hitachi H8/300H
        EM_H8S = 48,      //Hitachi H8S
        EM_H8_500 = 49,   //Hitachi H8/500
        EM_IA_64 = 50,    //Intel IA-64 processor architecture
        EM_MIPS_X = 51,   //Stanford MIPS-X
        EM_COLDFIRE = 52, //	Motorola ColdFire
        EM_68HC12 = 53,   //Motorola M68HC12
        EM_MMA = 54,      //Fujitsu MMA Multimedia Accelerator
        EM_PCP = 55,      //Siemens PCP
        EM_NCPU = 56,     //Sony nCPU embedded RISC processor
        EM_NDR1 = 57,     //Denso NDR1 microprocessor
        EM_STARCORE = 58, // 	Motorola Star*Core processor
        EM_ME16 = 59,     //Toyota ME16 processor
        EM_ST100 = 60,    //STMicroelectronics ST100 processor
        EM_TINYJ = 61,    //Advanced Logic Corp. TinyJ embedded processor family
        //Reserved 	62-65 	//Reserved for future use
        EM_FX66 = 66,     //Siemens FX66 microcontroller
        EM_ST9PLUS = 67,  //	STMicroelectronics ST9+ 8/16 bit microcontroller
        EM_ST7 = 68,      //STMicroelectronics ST7 8-bit microcontroller
        EM_68HC16 = 69,   //	Motorola MC68HC16 Microcontroller
        EM_68HC11 = 70,   //	Motorola MC68HC11 Microcontroller
        EM_68HC08 = 71,   //	Motorola MC68HC08 Microcontroller
        EM_68HC05 = 72,   //	Motorola MC68HC05 Microcontroller
        EM_SVX = 73,      //Silicon Graphics SVx
        EM_ST19 = 74,     //	STMicroelectronics ST19 8-bit microcontroller
        EM_VAX = 75,      //	Digital VAX
        EM_CRIS = 76,     //	Axis Communications 32-bit embedded processor
        EM_JAVELIN = 77,  //Infineon Technologies 32-bit embedded processor
        EM_FIREPATH = 78, // 	Element 14 64-bit DSP Processor
        EM_ZSP = 79,      //LSI Logic 16-bit DSP Processor
        EM_MMIX = 80,     // 	Donald Knuth's educational 64-bit processor
        EM_HUANY = 81,    //	Harvard University machine-independent object files
        EM_PRISM = 82,    //	SiTera Prism
        EM_AVR = 83,           /* Atmel AVR 8-bit microcontroller */
        EM_FR30 = 84,          /* Fujitsu FR30 */
        EM_D10V = 85,          /* Mitsubishi D10V */
        EM_D30V = 86,          /* Mitsubishi D30V */
        EM_V850 = 87,          /* NEC v850 */
        EM_M32R = 88,          /* Mitsubishi M32R */
        EM_MN10300 = 89,       /* Matsushita MN10300 */
        EM_MN10200 = 90,       /* Matsushita MN10200 */
        EM_PJ = 91,            /* picoJava */
        EM_OPENRISC = 92,      /* OpenRISC 32-bit embedded processor */
        EM_ARC_COMPACT = 93,   /* ARC International ARCompact */
        EM_XTENSA = 94,        /* Tensilica Xtensa Architecture */
        EM_VIDEOCORE = 95,     /* Alphamosaic VideoCore */
        EM_TMM_GPP = 96,       /* Thompson Multimedia General Purpose Proc */
        EM_NS32K = 97,         /* National Semi. 32000 */
        EM_TPC = 98,           /* Tenor Network TPC */
        EM_SNP1K = 99,         /* Trebia SNP 1000 */
        EM_ST200 = 100,        /* STMicroelectronics ST200 */
        EM_IP2K = 101,         /* Ubicom IP2xxx */
        EM_MAX = 102,          /* MAX processor */
        EM_CR = 103,           /* National Semi. CompactRISC */
        EM_F2MC16 = 104,       /* Fujitsu F2MC16 */
        EM_MSP430 = 105,       /* Texas Instruments msp430 */
        EM_BLACKFIN = 106,     /* Analog Devices Blackfin DSP */
        EM_SE_C33 = 107,       /* Seiko Epson S1C33 family */
        EM_SEP = 108,          /* Sharp embedded microprocessor */
        EM_ARCA = 109,         /* Arca RISC */
        EM_UNICORE = 110,      /* PKU-Unity & MPRC Peking Uni. mc series */
        EM_EXCESS = 111,       /* eXcess configurable cpu */
        EM_DXP = 112,          /* Icera Semi. Deep Execution Processor */
        EM_ALTERA_NIOS2 = 113, /* Altera Nios II */
        EM_CRX = 114,          /* National Semi. CompactRISC CRX */
        EM_XGATE = 115,        /* Motorola XGATE */
        EM_C166 = 116,         /* Infineon C16x/XC16x */
        EM_M16C = 117,         /* Renesas M16C */
        EM_DSPIC30F = 118,     /* Microchip Technology dsPIC30F */
        EM_CE = 119,           /* Freescale Communication Engine RISC */
        EM_M32C = 120,         /* Renesas M32C */
        /* reserved 121-130 */
        EM_TSK3000 = 131,       /* Altium TSK3000 */
        EM_RS08 = 132,          /* Freescale RS08 */
        EM_SHARC = 133,         /* Analog Devices SHARC family */
        EM_ECOG2 = 134,         /* Cyan Technology eCOG2 */
        EM_SCORE7 = 135,        /* Sunplus S+core7 RISC */
        EM_DSP24 = 136,         /* New Japan Radio (NJR) 24-bit DSP */
        EM_VIDEOCORE3 = 137,    /* Broadcom VideoCore III */
        EM_LATTICEMICO32 = 138, /* RISC for Lattice FPGA */
        EM_SE_C17 = 139,        /* Seiko Epson C17 */
        EM_TI_C6000 = 140,      /* Texas Instruments TMS320C6000 DSP */
        EM_TI_C2000 = 141,      /* Texas Instruments TMS320C2000 DSP */
        EM_TI_C5500 = 142,      /* Texas Instruments TMS320C55x DSP */
        EM_TI_ARP32 = 143,      /* Texas Instruments App. Specific RISC */
        EM_TI_PRU = 144,        /* Texas Instruments Prog. Realtime Unit */
        /* reserved 145-159 */
        EM_MMDSP_PLUS = 160,  /* STMicroelectronics 64bit VLIW DSP */
        EM_CYPRESS_M8C = 161, /* Cypress M8C */
        EM_R32C = 162,        /* Renesas R32C */
        EM_TRIMEDIA = 163,    /* NXP Semi. TriMedia */
        EM_QDSP6 = 164,       /* QUALCOMM DSP6 */
        EM_8051 = 165,        /* Intel 8051 and variants */
        EM_STXP7X = 166,      /* STMicroelectronics STxP7x */
        EM_NDS32 = 167,       /* Andes Tech. compact code emb. RISC */
        EM_ECOG1X = 168,      /* Cyan Technology eCOG1X */
        EM_MAXQ30 = 169,      /* Dallas Semi. MAXQ30 mc */
        EM_XIMO16 = 170,      /* New Japan Radio (NJR) 16-bit DSP */
        EM_MANIK = 171,       /* M2000 Reconfigurable RISC */
        EM_CRAYNV2 = 172,     /* Cray NV2 vector architecture */
        EM_RX = 173,          /* Renesas RX */
        EM_METAG = 174,       /* Imagination Tech. META */
        EM_MCST_ELBRUS = 175, /* MCST Elbrus */
        EM_ECOG16 = 176,      /* Cyan Technology eCOG16 */
        EM_CR16 = 177,        /* National Semi. CompactRISC CR16 */
        EM_ETPU = 178,        /* Freescale Extended Time Processing Unit */
        EM_SLE9X = 179,       /* Infineon Tech. SLE9X */
        EM_L10M = 180,        /* Intel L10M */
        EM_K10M = 181,        /* Intel K10M */
        /* reserved 182 */
        EM_AARCH64 = 183, /* ARM AARCH64 */
        /* reserved 184 */
        EM_AVR32 = 185,        /* Amtel 32-bit microprocessor */
        EM_STM8 = 186,         /* STMicroelectronics STM8 */
        EM_TILE64 = 187,       /* Tileta TILE64 */
        EM_TILEPRO = 188,      /* Tilera TILEPro */
        EM_MICROBLAZE = 189,   /* Xilinx MicroBlaze */
        EM_CUDA = 190,         /* NVIDIA CUDA */
        EM_TILEGX = 191,       /* Tilera TILE-Gx */
        EM_CLOUDSHIELD = 192,  /* CloudShield */
        EM_COREA_1ST = 193,    /* KIPO-KAIST Core-A 1st gen. */
        EM_COREA_2ND = 194,    /* KIPO-KAIST Core-A 2nd gen. */
        EM_ARC_COMPACT2 = 195, /* Synopsys ARCompact V2 */
        EM_OPEN8 = 196,        /* Open8 RISC */
        EM_RL78 = 197,         /* Renesas RL78 */
        EM_VIDEOCORE5 = 198,   /* Broadcom VideoCore V */
        EM_78KOR = 199,        /* Renesas 78KOR */
        EM_56800EX = 200,      /* Freescale 56800EX DSC */
        EM_BA1 = 201,          /* Beyond BA1 */
        EM_BA2 = 202,          /* Beyond BA2 */
        EM_XCORE = 203,        /* XMOS xCORE */
        EM_MCHP_PIC = 204,     /* Microchip 8-bit PIC(r) */
        /* reserved 205-209 */
        EM_KM32 = 210,        /* KM211 KM32 */
        EM_KMX32 = 211,       /* KM211 KMX32 */
        EM_EMX16 = 212,       /* KM211 KMX16 */
        EM_EMX8 = 213,        /* KM211 KMX8 */
        EM_KVARC = 214,       /* KM211 KVARC */
        EM_CDP = 215,         /* Paneve CDP */
        EM_COGE = 216,        /* Cognitive Smart Memory Processor */
        EM_COOL = 217,        /* Bluechip CoolEngine */
        EM_NORC = 218,        /* Nanoradio Optimized RISC */
        EM_CSR_KALIMBA = 219, /* CSR Kalimba */
        EM_Z80 = 220,         /* Zilog Z80 */
        EM_VISIUM = 221,      /* Controls and Data Services VISIUMcore */
        EM_FT32 = 222,        /* FTDI Chip FT32 */
        EM_MOXIE = 223,       /* Moxie processor */
        EM_AMDGPU = 224,      /* AMD GPU */
        /* reserved 225-242 */
        EM_RISCV = 243, /* RISC-V */
        EM_BPF = 247,   /* Linux BPF -- in-kernel virtual machine */
        EM_CSKY = 252,  /* C-SKY */
        EM_NUM = 253,
        /* Old spellings/synonyms.  */
           //  EM_ARC_A5    =    EM_ARC_COMPACT,
            /* If it is necessary to assign new unofficial EM_* values, please
               pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
               chances of collision with official or non-GNU unofficial values.  */
           //EM_ALPHA     =   0x9026,

    }
    impl EI_MACH {

        pub fn as_u16(&self) -> u16 {
            match self {
                EI_MACH::EM_NONE => 0,
                EI_MACH::EM_M32 => 1,          //	AT&T WE 32100
                EI_MACH::EM_SPARC => 2,        //SPARC
                EI_MACH::EM_386 => 3,          //Intel 80386
                EI_MACH::EM_68K => 4,          //Motorola 68000
                EI_MACH::EM_88K => 5,          //	Motorola 88000
               // RESERVED  6          // 	Reserved for future use
                EI_MACH::EM_860 => 7,          //Intel 80860
                EI_MACH::EM_MIPS => 8,         //	MIPS I Architecture
                EI_MACH::EM_S370 => 9,         // 	IBM System/370 Processor
                EI_MACH::EM_MIPS_RS3_LE => 10, //	MIPS RS3000 Little-endian
                //RESERVED 	11-14 //	Reserved for future use
                EI_MACH::EM_PARISC => 15, //Hewlett-Packard PA-RISC
                //RESERVED 	16 //	Reserved for future use
                EI_MACH::EM_VPP500 => 17,      //Fujitsu VPP500
                EI_MACH::EM_SPARC32PLUS => 18, //Enhanced instruction set SPARC
                EI_MACH::EM_960 => 19,         //	Intel 80960
                EI_MACH::EM_PPC => 20,         //	PowerPC
                EI_MACH::EM_PPC64 => 21,       //	64-bit PowerPC
                //RESERVED 	22-35 //	Reserved for future use
                EI_MACH::EM_V800 => 36,     //	NEC V800
                EI_MACH::EM_FR20 => 37,     //	Fujitsu FR20
                EI_MACH::EM_RH32 => 38,     //TRW RH-32
                EI_MACH::EM_RCE => 39,      //Motorola RCE
                EI_MACH::EM_ARM => 40,      //Advanced RISC Machines ARM
                EI_MACH::EM_ALPHA => 41,    //	Digital Alpha
                EI_MACH::EM_SH => 42,       //Hitachi SH
                EI_MACH::EM_SPARCV9 => 43,  // 	SPARC Version 9
                EI_MACH::EM_TRICORE => 44,  //	Siemens Tricore embedded processor
                EI_MACH::EM_ARC => 45,      //Argonaut RISC Core, Argonaut Technologies Inc.
                EI_MACH::EM_H8_300 => 46,   //	Hitachi H8/300
                EI_MACH::EM_H8_300H => 47,  //	Hitachi H8/300H
                EI_MACH::EM_H8S => 48,      //Hitachi H8S
                EI_MACH::EM_H8_500 => 49,   //Hitachi H8/500
                EI_MACH::EM_IA_64 => 50,    //Intel IA-64 processor architecture
                EI_MACH::EM_MIPS_X => 51,   //Stanford MIPS-X
                EI_MACH::EM_COLDFIRE => 52, //	Motorola ColdFire
                EI_MACH::EM_68HC12 => 53,   //Motorola M68HC12
                EI_MACH::EM_MMA => 54,      //Fujitsu MMA Multimedia Accelerator
                EI_MACH::EM_PCP => 55,      //Siemens PCP
                EI_MACH:: EM_NCPU => 56,     //Sony nCPU embedded RISC processor
                EI_MACH::EM_NDR1 => 57,     //Denso NDR1 microprocessor
                EI_MACH::EM_STARCORE => 58, // 	Motorola Star*Core processor
                EI_MACH::EM_ME16 => 59,     //Toyota ME16 processor
                EI_MACH::EM_ST100 => 60,    //STMicroelectronics ST100 processor
                EI_MACH::EM_TINYJ => 61,    //Advanced Logic Corp. TinyJ embedded processor family
                //Reserved 	62-65 	//Reserved for future use
                EI_MACH::EM_FX66 => 66,     //Siemens FX66 microcontroller
                EI_MACH::EM_ST9PLUS => 67,  //	STMicroelectronics ST9+ 8/16 bit microcontroller
                EI_MACH::EM_ST7 => 68,      //STMicroelectronics ST7 8-bit microcontroller
                EI_MACH::EM_68HC16 => 69,   //	Motorola MC68HC16 Microcontroller
                EI_MACH::EM_68HC11 => 70,   //	Motorola MC68HC11 Microcontroller
                EI_MACH::EM_68HC08 => 71,   //	Motorola MC68HC08 Microcontroller
                EI_MACH::EM_68HC05 => 72,   //	Motorola MC68HC05 Microcontroller
                EI_MACH::EM_SVX => 73,      //Silicon Graphics SVx
                EI_MACH::EM_ST19 => 74,     //	STMicroelectronics ST19 8-bit microcontroller
                EI_MACH::EM_VAX => 75,      //	Digital VAX
                EI_MACH::EM_CRIS => 76,     //	Axis Communications 32-bit embedded processor
                EI_MACH::EM_JAVELIN => 77,  //Infineon Technologies 32-bit embedded processor
                EI_MACH::EM_FIREPATH => 78, // 	Element 14 64-bit DSP Processor
                EI_MACH::EM_ZSP => 79,      //LSI Logic 16-bit DSP Processor
                EI_MACH::EM_MMIX => 80,     // 	Donald Knuth's educational 64-bit processor
                EI_MACH::EM_HUANY => 81,    //	Harvard University machine-independent object files
                EI_MACH::EM_PRISM => 82,    //	SiTera Prism
                EI_MACH::EM_AVR => 83,           /* Atmel AVR 8-bit microcontroller */
                EI_MACH::EM_FR30 => 84,          /* Fujitsu FR30 */
                EI_MACH::EM_D10V => 85,          /* Mitsubishi D10V */
                EI_MACH::EM_D30V => 86,          /* Mitsubishi D30V */
                EI_MACH::EM_V850 => 87,          /* NEC v850 */
                EI_MACH::EM_M32R => 88,          /* Mitsubishi M32R */
                EI_MACH::EM_MN10300 => 89,       /* Matsushita MN10300 */
                EI_MACH::EM_MN10200 => 90,       /* Matsushita MN10200 */
                EI_MACH::EM_PJ => 91,            /* picoJava */
                EI_MACH::EM_OPENRISC => 92,      /* OpenRISC 32-bit embedded processor */
                EI_MACH::EM_ARC_COMPACT => 93,   /* ARC International ARCompact */
                EI_MACH::EM_XTENSA => 94,        /* Tensilica Xtensa Architecture */
                EI_MACH::EM_VIDEOCORE => 95,     /* Alphamosaic VideoCore */
                EI_MACH::EM_TMM_GPP => 96,       /* Thompson Multimedia General Purpose Proc */
                EI_MACH::EM_NS32K => 97,         /* National Semi. 32000 */
                EI_MACH::EM_TPC => 98,           /* Tenor Network TPC */
                EI_MACH::EM_SNP1K => 99,         /* Trebia SNP 1000 */
                EI_MACH::EM_ST200 => 100,        /* STMicroelectronics ST200 */
                EI_MACH::EM_IP2K => 101,         /* Ubicom IP2xxx */
                EI_MACH::EM_MAX => 102,          /* MAX processor */
                EI_MACH::EM_CR => 103,           /* National Semi. CompactRISC */
                EI_MACH::EM_F2MC16 => 104,       /* Fujitsu F2MC16 */
                EI_MACH::EM_MSP430 => 105,       /* Texas Instruments msp430 */
                EI_MACH::EM_BLACKFIN => 106,     /* Analog Devices Blackfin DSP */
                EI_MACH::EM_SE_C33 => 107,       /* Seiko Epson S1C33 family */
                EI_MACH::EM_SEP => 108,          /* Sharp embedded microprocessor */
                EI_MACH::EM_ARCA => 109,         /* Arca RISC */
                EI_MACH::EM_UNICORE => 110,      /* PKU-Unity & MPRC Peking Uni. mc series */
                EI_MACH::EM_EXCESS => 111,       /* eXcess configurable cpu */
                EI_MACH::EM_DXP => 112,          /* Icera Semi. Deep Execution Processor */
                EI_MACH::EM_ALTERA_NIOS2 => 113, /* Altera Nios II */
                EI_MACH::EM_CRX => 114,          /* National Semi. CompactRISC CRX */
                EI_MACH::EM_XGATE => 115,        /* Motorola XGATE */
                EI_MACH::EM_C166 => 116,         /* Infineon C16x/XC16x */
                EI_MACH::EM_M16C => 117,         /* Renesas M16C */
                EI_MACH::EM_DSPIC30F => 118,     /* Microchip Technology dsPIC30F */
                EI_MACH::EM_CE => 119,           /* Freescale Communication Engine RISC */
                EI_MACH::EM_M32C => 120,         /* Renesas M32C */
                /* reserved 121-130 */
                EI_MACH::EM_TSK3000 => 131,       /* Altium TSK3000 */
                EI_MACH::EM_RS08 => 132,          /* Freescale RS08 */
                EI_MACH::EM_SHARC => 133,         /* Analog Devices SHARC family */
                EI_MACH::EM_ECOG2 => 134,         /* Cyan Technology eCOG2 */
                EI_MACH::EM_SCORE7 => 135,        /* Sunplus S+core7 RISC */
                EI_MACH::EM_DSP24 => 136,         /* New Japan Radio (NJR) 24-bit DSP */
                EI_MACH::EM_VIDEOCORE3 => 137,    /* Broadcom VideoCore III */
                EI_MACH::EM_LATTICEMICO32 => 138, /* RISC for Lattice FPGA */
                EI_MACH::EM_SE_C17 => 139,        /* Seiko Epson C17 */
                EI_MACH::EM_TI_C6000 => 140,      /* Texas Instruments TMS320C6000 DSP */
                EI_MACH::EM_TI_C2000 => 141,      /* Texas Instruments TMS320C2000 DSP */
                EI_MACH::EM_TI_C5500 => 142,      /* Texas Instruments TMS320C55x DSP */
                EI_MACH::EM_TI_ARP32 => 143,      /* Texas Instruments App. Specific RISC */
                EI_MACH::EM_TI_PRU => 144,        /* Texas Instruments Prog. Realtime Unit */
                /* reserved 145-159 */
                EI_MACH::EM_MMDSP_PLUS => 160,  /* STMicroelectronics 64bit VLIW DSP */
                EI_MACH::EM_CYPRESS_M8C => 161, /* Cypress M8C */
                EI_MACH::EM_R32C => 162,        /* Renesas R32C */
                EI_MACH::EM_TRIMEDIA => 163,    /* NXP Semi. TriMedia */
                EI_MACH::EM_QDSP6 => 164,       /* QUALCOMM DSP6 */
                EI_MACH::EM_8051 => 165,        /* Intel 8051 and variants */
                EI_MACH::EM_STXP7X => 166,      /* STMicroelectronics STxP7x */
                EI_MACH::EM_NDS32 => 167,       /* Andes Tech. compact code emb. RISC */
                EI_MACH::EM_ECOG1X => 168,      /* Cyan Technology eCOG1X */
                EI_MACH::EM_MAXQ30 => 169,      /* Dallas Semi. MAXQ30 mc */
                EI_MACH::EM_XIMO16 => 170,      /* New Japan Radio (NJR) 16-bit DSP */
                EI_MACH::EM_MANIK => 171,       /* M2000 Reconfigurable RISC */
                EI_MACH::EM_CRAYNV2 => 172,     /* Cray NV2 vector architecture */
                EI_MACH::EM_RX => 173,          /* Renesas RX */
                EI_MACH::EM_METAG => 174,       /* Imagination Tech. META */
                EI_MACH::EM_MCST_ELBRUS => 175, /* MCST Elbrus */
                EI_MACH::EM_ECOG16 => 176,      /* Cyan Technology eCOG16 */
                EI_MACH::EM_CR16 => 177,        /* National Semi. CompactRISC CR16 */
                EI_MACH::EM_ETPU => 178,        /* Freescale Extended Time Processing Unit */
                EI_MACH::EM_SLE9X => 179,       /* Infineon Tech. SLE9X */
                EI_MACH::EM_L10M => 180,        /* Intel L10M */
                EI_MACH::EM_K10M => 181,        /* Intel K10M */
                /* reserved 182 */
                EI_MACH::EM_AARCH64 => 183, /* ARM AARCH64 */
                /* reserved 184 */
                EI_MACH::EM_AVR32 => 185,        /* Amtel 32-bit microprocessor */
                EI_MACH::EM_STM8 => 186,         /* STMicroelectronics STM8 */
                EI_MACH::EM_TILE64 => 187,       /* Tileta TILE64 */
                EI_MACH::EM_TILEPRO => 188,      /* Tilera TILEPro */
                EI_MACH::EM_MICROBLAZE => 189,   /* Xilinx MicroBlaze */
                EI_MACH::EM_CUDA => 190,         /* NVIDIA CUDA */
                EI_MACH::EM_TILEGX => 191,       /* Tilera TILE-Gx */
                EI_MACH::EM_CLOUDSHIELD => 192,  /* CloudShield */
                EI_MACH::EM_COREA_1ST => 193,    /* KIPO-KAIST Core-A 1st gen. */
                EI_MACH::EM_COREA_2ND => 194,    /* KIPO-KAIST Core-A 2nd gen. */
                EI_MACH::EM_ARC_COMPACT2 => 195, /* Synopsys ARCompact V2 */
                EI_MACH::EM_OPEN8 => 196,        /* Open8 RISC */
                EI_MACH::EM_RL78 => 197,         /* Renesas RL78 */
                EI_MACH::EM_VIDEOCORE5 => 198,   /* Broadcom VideoCore V */
                EI_MACH::EM_78KOR => 199,        /* Renesas 78KOR */
                EI_MACH::EM_56800EX => 200,      /* Freescale 56800EX DSC */
                EI_MACH::EM_BA1 => 201,          /* Beyond BA1 */
                EI_MACH::EM_BA2 => 202,          /* Beyond BA2 */
                EI_MACH::EM_XCORE => 203,        /* XMOS xCORE */
                EI_MACH::EM_MCHP_PIC => 204,     /* Microchip 8-bit PIC(r) */
                /* reserved 205-209 */
                EI_MACH::EM_KM32 => 210,        /* KM211 KM32 */
                EI_MACH::EM_KMX32 => 211,       /* KM211 KMX32 */
                EI_MACH::EM_EMX16 => 212,       /* KM211 KMX16 */
                EI_MACH::EM_EMX8 => 213,        /* KM211 KMX8 */
                EI_MACH::EM_KVARC => 214,       /* KM211 KVARC */
                EI_MACH::EM_CDP => 215,         /* Paneve CDP */
                EI_MACH::EM_COGE => 216,        /* Cognitive Smart Memory Processor */
                EI_MACH::EM_COOL => 217,        /* Bluechip CoolEngine */
                EI_MACH::EM_NORC => 218,        /* Nanoradio Optimized RISC */
                EI_MACH::EM_CSR_KALIMBA => 219, /* CSR Kalimba */
                EI_MACH::EM_Z80 => 220,         /* Zilog Z80 */
                EI_MACH::EM_VISIUM => 221,      /* Controls and Data Services VISIUMcore */
                EI_MACH::EM_FT32 => 222,        /* FTDI Chip FT32 */
                EI_MACH::EM_MOXIE => 223,       /* Moxie processor */
                EI_MACH::EM_AMDGPU => 224,      /* AMD GPU */
                /* reserved 225-242 */
                EI_MACH::EM_RISCV => 243, /* RISC-V */
                EI_MACH::EM_BPF => 247,   /* Linux BPF -- in-kernel virtual machine */
                EI_MACH::EM_CSKY => 252,  /* C-SKY */
                EI_MACH::EM_NUM => 253,
                /* Old spellings/synonyms.  */
                //  EM_ARC_A5    =    EM_ARC_COMPACT,
                /* If it is necessary to assign new unofficial EM_* values, please
                   pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
                   chances of collision with official or non-GNU unofficial values.  */
                //    EM_ALPHA     =>   0x9026,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum EI_CLASS {
        ELFCLASSNONE,
        ELFCLASS32,
        ELFCLASS64,
        ELFCLASSOTHER(u8),
    }

    impl EI_CLASS {
        pub fn as_u8(&self) -> u8 {
            match self {
                EI_CLASS::ELFCLASSNONE => 0,
                EI_CLASS::ELFCLASS32 => 1,
                EI_CLASS::ELFCLASS64 => 2,
                EI_CLASS::ELFCLASSOTHER(d) => *d,
            }
        }
    }

    #[repr(u8)]
    #[derive(Clone, Copy, Debug)]
    pub enum EI_DATA {
        ELFDATANONE = 0,
        ELFDATA2LSB,
        ELFDATA2MSB,
        ELFDATAOTHER(u8), //TODO are other values than 3 valid here? Need to check ELF Specs for arm and x86
    }

    impl EI_DATA {
        pub fn as_u8(&self) -> u8 {
            match self {
                EI_DATA::ELFDATANONE => 0,
                EI_DATA::ELFDATA2LSB => 1,
                EI_DATA::ELFDATA2MSB => 2,
                EI_DATA::ELFDATAOTHER(d) => *d,
            }
        }
    }

    //TODO figure out different system so that aliasing is allowed?
    #[repr(u8)]
    #[allow(non_camel_case_types)]
    pub enum EI_OSABI {

        ELFOSABI_NONE       =     0,                /* UNIX System V ABI */
        //ELFOSABI_SYSV =                            /* Alias ELFOSABI_NONE,  */
        ELFOSABI_HPUX =            1,                /* HP-UX */
        ELFOSABI_NETBSD =              2,            /* NetBSD.  */
        ELFOSABI_GNU  =           3,                /* Object uses GNU ELF extensions.  */
        //ELFOSABI_LINUX =            ,             /* Compatibility alias ELFOSABI_GNU  */
        ELFOSABI_SOLARIS =      6,                  /* Sun Solaris.  */
        ELFOSABI_AIX =          7,                  /* IBM AIX.  */
        ELFOSABI_IRIX =           8,                /* SGI Irix.  */
        ELFOSABI_FREEBSD =     9,                   /* FreeBSD.  */
        ELFOSABI_TRU64 =            10,             /* Compaq TRU64 UNIX.  */
        ELFOSABI_MODESTO =     11,                  /* Novell Modesto.  */
        ELFOSABI_OPENBSD =     12,                  /* OpenBSD.  */
        ELFOSABI_ARM_AEABI  =      64,              /* ARM EABI */
        ELFOSABI_ARM  =        97,                  /* ARM */
        ELFOSABI_OTHER(u8),
        ELFOSABI_STANDALONE  =      255,        /* Standalone (embedded) application */
    }

    #[allow(non_camel_case_types)]
    impl EI_OSABI {
        pub fn as_u8(&self) -> u8 {
            match self {
                EI_OSABI::ELFOSABI_NONE => 0,         /* UNIX System V ABI */
                //EI_OSABI::ELFOSABI_SYSV => 0,         //EI_OSABI::ELFOSABI_NONE,        /* Alias.  */
                EI_OSABI::ELFOSABI_HPUX => 1,         /* HP-UX */
                EI_OSABI::ELFOSABI_NETBSD => 2,       /* NetBSD.  */
                EI_OSABI::ELFOSABI_GNU => 3,          /* Object uses GNU ELF extensions.  */
               // EI_OSABI::ELFOSABI_LINUX => 3,        //EI_OSABI::ELFOSABI_GNU, /* Compatibility alias.  */
                EI_OSABI::ELFOSABI_SOLARIS => 6,      /* Sun Solaris.  */
                EI_OSABI::ELFOSABI_AIX => 7,          /* IBM AIX.  */
                EI_OSABI::ELFOSABI_IRIX => 8,         /* SGI Irix.  */
                EI_OSABI::ELFOSABI_FREEBSD => 9,      /* FreeBSD.  */
                EI_OSABI::ELFOSABI_TRU64 => 10,       /* Compaq TRU64 UNIX.  */
                EI_OSABI::ELFOSABI_MODESTO => 11,     /* Novell Modesto.  */
                EI_OSABI::ELFOSABI_OPENBSD => 12,     /* OpenBSD.  */
                EI_OSABI::ELFOSABI_ARM_AEABI => 64,   /* ARM EABI */
                EI_OSABI::ELFOSABI_ARM => 97,         /* ARM */
                EI_OSABI::ELFOSABI_OTHER(d) => *d,
                EI_OSABI::ELFOSABI_STANDALONE => 255, /* Standalone (embedded) application */
            }
        }
    }

    pub enum EI_VERS{
        EV_NONE = 0,
        EV_CURRENT = 1,
    }


}

