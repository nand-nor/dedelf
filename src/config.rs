use std::env;
use std::str::FromStr;
use std::process::exit;
use std::io::{stdout, stderr};
use argparse::{ArgumentParser, StoreTrue, StoreFalse, Store, List, StoreOption};

/* Default injection mode values */
const INJ_DEFAULT_SIZE: usize = 0x1000;
const INJ_DEFAULT_EXT: Option<&'static str> = Some(".text");
const INJ_DEFAULT_ENTRY: Option<u64> = None;
const INJ_DEFAULT_OFFSET: Option<u64> = None;
const INJ_DEFAULT_REPLACE: bool = false;
/*
* Parse args such that a user can enter args in any order. All args other than input file
* and modification type are optional for injection mode; for modification mode, all args are
* required.
*
* TODO: implement support for config file parsing
*/
pub fn parse_args(infile: &mut String,
                  outfile: &mut String,
                  options: &mut DedElfOps) -> Result<(), std::io::Error>{

    let mut size: Option<String> = None;//Some(INJ_DEFAULT_SIZE);
    let mut entry: Option<String> = None;//INJ_DEFAULT_ENTRY;
    let mut extend: Option<String> = None;
    let mut offset: Option<String> = None;//INJ_DEFAULT_OFFSET;
    let mut replace = INJ_DEFAULT_REPLACE;
    let mut inj_file: String = " ".to_string();
    let mut toutfile: String = " ".to_string();
    let mut field: String = " ".to_string();
    let mut replace_field: String = " ".to_string();
    let mut mod_mode: ModOps = ModOps::EXEC;

    let mut default_mode: Mode = Mode::INJECT;

    {
        let mut parser = ArgumentParser::new();

        parser.refer(&mut default_mode).required().add_argument("mode", Store,
                                                                r#"Specify run mode: either `inject` or `modify`"#);
        parser.refer(infile).required().add_argument("infile", Store,
                                                     r#"File to modify or inject"#);

        parser.refer(&mut inj_file)
            .add_option(&["-i", "--infile"], Store,
                          r#"INJECTION MODE: Provide a file containing the bytes to inject. File should be trimmed to contain ony desired bytes"#);

        parser.refer(&mut mod_mode)
            .add_option(&["-m", "--mod"], Store,
                        r#"MODIFY MODE: Provide a file containing the bytes to inject. File should be trimmed to contain ony desired bytes"#);

        parser.refer(&mut toutfile)
            .add_option(&["-o", "--outfile"], Store,
                        r#"INJECTION or MODIFY MODE: Provide a file containing to write modified bytes to"#);

        parser.refer(&mut size)
            .add_option(&["-s", "--size"], StoreOption,
                        r#"INJECTION MODE: Specify byte size (will be rounded to next closest size in pages (4k))"#);
        parser.refer(&mut entry)
            .add_option(&["-e", "--entry"], StoreOption,
                        r#"INJECTION MODE: Specify if entry point in exec header should be modified to provided byte offset"#);

        parser.refer(&mut extend)
            .add_option(&["-p", "--position"], StoreOption,
                        r#"INJECTION or MODIFY MODE: Specify section. If injection mode, this will specify if bytes should be injected at end of provided section (can be name or index). If modify mode, must be used for either sec_header or prog_header options. Use this to specify the name (for sections) or index (for either sections or segments) of the header to be modified"#);

        parser.refer(&mut offset)
            .add_option(&["-b", "--offset"], StoreOption,
                        r#"INJECTION MODE: Specify if bytes should be injected at exact byte offset provided"#);

        parser.refer(&mut replace)
            .add_option(&["--overwrite"], StoreTrue,
                        r#"INJECTION MODE: Specify if bytes should be injected to replace the entire section"#);

        parser.refer(&mut field)
            .add_option(&["-f", "--field"], Store,
                        r#"MODIFY MODE: Specify header field to modify"#);
        parser.refer(&mut replace_field)
            .add_option(&["-r", "--replace"], Store,
                        r#"MODIFY MODE: Specify value to replace field with"#);

        parser.parse_args_or_exit();
    }

    //Set default outfile name if none provided. Default outfile name is _inj
    // appended to original file name
    if toutfile == " ".to_string(){
        let mut tfile = infile.clone();
        let temp = "_inj".to_string();
        tfile.push_str(&temp);
        *outfile = tfile.clone();
    } else {
        *outfile = toutfile.clone();
    }


    let copy = inj_file.to_string();
    match default_mode{
        Mode::INJECT => {
            if extend == None && offset == None {
                println!("Setting default inject section to {:?}", INJ_DEFAULT_EXT);
                extend = Some(INJ_DEFAULT_EXT.unwrap().to_string());
            }
            *options = DedElfOps::parse_inj_ops(size, extend, entry, replace, offset, inj_file)?;
            println!("OK options are: {:?}", *options);
            return Ok(())
        }
        Mode::MODIFY => {
            *options = DedElfOps::parse_mod_ops(mod_mode,field, replace_field, extend)?;
            return Ok(())
        }
        _ => {
            println!("Invalid CLI mode provided");
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           ("Invalid options provided")))
        }

    }
}


#[derive(Debug)]
pub struct DedElfOps {
    pub injection: Option<InjModeOps>,
    pub modify: Option<ModModeOps>,
}

impl DedElfOps {

    pub fn parse_inj_ops(
        size: Option<String>,
        extend: Option<String>,
        entry: Option<String>,
        replace: bool,
        b_offset: Option<String>,
        file: String) -> Result<DedElfOps, std::io::Error> {
        let mut op_flag = false;
        let mut new_size: usize = INJ_DEFAULT_SIZE;
        let mut new_entry: Option<u64> = INJ_DEFAULT_ENTRY;
        let mut new_extend: Option<String> = None;//Some(INJ_DEFAULT_EXT.unwrap().to_string());
        let mut new_b_offset: Option<u64> = INJ_DEFAULT_OFFSET;

        if let Some(size) = size {
            op_flag = true;

            let trimmed = size.trim_start_matches("0x");
            let check = usize::from_str_radix(trimmed, 16);
            if check.is_err() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               ("Invalid injection mode options (size) provided")))

            }
            new_size = check.unwrap();
        }
        if let Some(entry) = entry {
            op_flag = true;
            let trimmed = entry.trim_start_matches("0x");
            let check = u64::from_str_radix(trimmed, 16);
            if check.is_err() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               ("Invalid injection mode options (entry) provided")))
            }

            new_entry = Some(check.unwrap());
        }

        if let Some(extend) = extend {
            op_flag = true;
            new_extend = Some(extend);

        } else if extend.is_none() && b_offset.is_none() {
            new_extend = Some(INJ_DEFAULT_EXT.unwrap().to_string());
        }

        if let Some(b_offset) = b_offset {
            op_flag = true;
            let trimmed = b_offset.trim_start_matches("0x");
            let check = u64::from_str_radix(trimmed, 16);
            if check.is_err() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               ("Invalid injection mode options (byte offset) provided")))
            }
            new_b_offset = Some(check.unwrap());
        }



        match op_flag {
            false => {
                return Ok(DedElfOps {
                    injection: Some(InjModeOps::default(file)),
                    modify: None
                })
            }
            true => {
                return Ok(
                    DedElfOps{
                        injection: Some(InjModeOps {
                            file: file,
                            size: new_size,
                            extend: new_extend,
                            new_entry: new_entry,
                            replace: replace,
                            b_offset: new_b_offset,
                        }),
                        modify: None,
                })
            }
        }
    }

    pub fn no_ops() -> DedElfOps {
        DedElfOps {
            injection: None,
            modify: None,
        }
    }

    pub fn parse_mod_ops(op: ModOps, field: String, replacement: String,
                         placement: Option<String>) -> Result<DedElfOps, std::io::Error> {
        if field == " ".to_string() || replacement == " ".to_string(){
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           ("Invalid mod mode options provided")))
        }

        let mod_ops = match op {
            ModOps::EXEC => {
                ModModeOps {
                    exec: Some(ExecCfg {
                        op_mode: parse_exec_mod_ops(field)?,
                        replacement: replacement,
                    }),
                    sec: None,
                    seg: None,
                }
            }
            ModOps::SECTION => {
                if let Some(placement) = placement {
                    let cfg: SecCfg;
                    let trimmed = placement.trim_start_matches("0x");
                    let check = usize::from_str_radix(trimmed, 16);
                    if check.is_err() {
                        cfg = SecCfg {
                            op_mode: parse_sec_mod_ops(field)?,
                            sec_name: Some(placement),
                            sec_idx: None,
                            replacement: replacement,
                        };
                    } else {
                        cfg = SecCfg {
                            op_mode: parse_sec_mod_ops(field)?,
                            sec_name: None,
                            sec_idx: Some(check.unwrap()),
                            replacement: replacement,
                        };
                    }

                    ModModeOps {
                        exec: None,
                        sec: Some(cfg),
                        seg: None,
                    }
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                   ("Invalid mod mode options provided")))
                }
            }
            ModOps::SEGMENT => {
                if let Some(placement) = placement {

                    let trimmed = placement.trim_start_matches("0x");
                    let check = usize::from_str_radix(trimmed, 16);
                    if check.is_err() {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                       ("Invalid mod mode options provided")))
                    }
                    ModModeOps {
                        exec: None,
                        sec: None,
                        seg: Some(SegCfg {
                            op_mode: parse_seg_mod_ops(field)?,
                            seg_idx: check.unwrap(),
                            replacement: replacement,
                        }),
                    }
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                   ("Invalid mod mode options provided")))
                }
            }
        };

        Ok(DedElfOps {
            injection: None,
            modify: Some(mod_ops),
        })
    }

    pub fn get_inj_file(&self) -> Option<String> {
        if let Some(inj) = &self.injection {
            return Some(inj.file.clone());
        } else {
            return None;
        }
    }
}

pub enum Mode {
    MODIFY,
    INJECT,
    DUAL, //NOTE: not yet supported
}

impl FromStr for Mode {
    type Err = std::io::Error;
    fn from_str(mode: &str)->Result<Mode,std::io::Error>{
        match mode {
            "modify" => Ok(Mode::MODIFY),
            "inject" => Ok(Mode::INJECT),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                          ("Invalid mode option; use either `inject` or `modify`")))
        }
    }
}


impl FromStr for ModOps {
    type Err = std::io::Error;
    fn from_str(mode: &str)->Result<ModOps,std::io::Error>{
        match mode {
            "exec_header" => Ok(ModOps::EXEC),
            "sec_header" => Ok(ModOps::SECTION),
            "prog_header" => Ok(ModOps::SEGMENT),

//            "new_seg" => Ok(Mode::INJECT),
//            "new_sec" => Ok(Mode::MODIFY),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                ("Invalid modify option provided, use \
                                                one of the following: `exec_header`, \
                                                `sec_header`, `prog_header")))
        }
    }
}

#[derive(Clone, Debug)]
pub struct InjModeOps {
    file: String,
    size: usize,
    extend: Option<String>,
    b_offset: Option<u64>,
    replace: bool,
    new_entry: Option<u64>,
}

impl InjModeOps {

    fn default(file: String) -> InjModeOps {
        InjModeOps {
            file: file,
            size: INJ_DEFAULT_SIZE,
            extend: Some(INJ_DEFAULT_EXT.unwrap().to_string()),
            new_entry: INJ_DEFAULT_ENTRY,
            b_offset: INJ_DEFAULT_OFFSET,
            replace: INJ_DEFAULT_REPLACE,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_extend(&self) -> Option<String> {
        self.extend.clone()
    }

    pub fn get_file(&self) -> String {
        self.file.clone()
    }

    pub fn get_entry(&self) -> Option<u64> {
        self.new_entry
    }

    pub fn get_offset(&self)->Option<u64>{
        self.b_offset
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum ModOps {
    EXEC,
    SECTION,
    SEGMENT,
    //NEW_SEC,
    //NEW_SEG,
    // SYMBOL,
    // RELOC,
}

#[derive(Clone, Debug)]
pub struct ModModeOps {
    pub exec: Option<ExecCfg>,
    pub sec: Option<SecCfg>,
    pub seg: Option<SegCfg>,
}

#[derive(Clone, Debug)]
pub struct ExecCfg {
    pub op_mode: ExecModOps,
    pub replacement: String,
}

#[derive(Clone, Debug)]
pub struct SecCfg {
    pub op_mode: SecModOps,
    pub sec_name: Option<String>,
    pub sec_idx: Option<usize>,
    pub replacement: String,
}

#[derive(Clone, Debug)]
pub struct SegCfg {
    pub op_mode: SegModOps,
    pub seg_idx: usize,
    pub replacement: String,
}

#[derive(Copy, Clone, Debug)]
pub enum ExecModOps {
    IDENT,
    //CLASS,
    //DATA,
    //OSABI,
    TYPE,
    MACH,
    VERSION,
    ENTRY,
    PHOFF,
    SHOFF,
    FLAGS,
    EHSIZE,
    PHENTSIZE,
    PHNUM,
    SHENTSIZE,
    SHNUM,
    SHSTRNDX,
}

pub fn parse_exec_mod_ops(option: String) -> Result<ExecModOps, std::io::Error> {
    match option.as_str() {
        //"e_ident" => Ok(ExecModOps::IDENT),
        "EI_CLASS" => Ok(ExecModOps::IDENT),
        "EI_DATA" => Ok(ExecModOps::IDENT),
        "EI_OSABI" => Ok(ExecModOps::IDENT),

        "e_type" => Ok(ExecModOps::TYPE),
        "e_machine" => Ok(ExecModOps::MACH),
        "e_version" => Ok(ExecModOps::VERSION),
        "e_entry" => Ok(ExecModOps::ENTRY),
        "e_phoff" => Ok(ExecModOps::PHOFF),
        "e_shoff" => Ok(ExecModOps::SHOFF),
        "e_flags" => Ok(ExecModOps::FLAGS),
        "e_ehsize" => Ok(ExecModOps::EHSIZE),
        "e_phentsize" => Ok(ExecModOps::PHENTSIZE),
        "e_phnum" => Ok(ExecModOps::PHNUM),
        "e_shentsize" => Ok(ExecModOps::SHENTSIZE),
        "e_shnum" => Ok(ExecModOps::SHNUM),
        "e_shstrndx" => Ok(ExecModOps::SHSTRNDX),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            ("Invalid modify option provided for exec header field")))
    }
}

pub fn get_exec_field(option: ExecModOps) -> String {
    match option {
        ExecModOps::IDENT => {
            "e_ident".to_string()
        }
        ExecModOps::TYPE => {
            "e_type".to_string()
        }
        ExecModOps::MACH => {
            "e_machine".to_string()
        }
        ExecModOps::VERSION => {
            "e_version".to_string()
        }
        ExecModOps::ENTRY => {
            "e_entry".to_string()
        }
        ExecModOps::PHOFF => {
            "e_phoff".to_string()
        }
        ExecModOps::SHOFF => {
            "e_shoff".to_string()
        }
        ExecModOps::FLAGS => {
            "e_flags".to_string()
        }
        ExecModOps::EHSIZE => {
            "e_ehsize".to_string()
        }
        ExecModOps::PHENTSIZE => {
            "e_phentsize".to_string()
        }
        ExecModOps::PHNUM => {
            "e_phnum".to_string()
        }
        ExecModOps::SHENTSIZE => {
            "e_shentsize".to_string()
        }
        ExecModOps::SHNUM => {
            "e_shnum".to_string()
        }
        ExecModOps::SHSTRNDX => {
            "e_shstrndx".to_string()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SecModOps {
    NAME,
    TYPE,
    FLAGS,
    ADDR,
    OFFSET,
    SIZE,
    LINK,
    INFO,
    ADDRALIGN,
    ENTSIZE,
}

pub fn parse_sec_mod_ops(option: String) -> Result<SecModOps, std::io::Error> {
    match option.as_str() {
        "sh_name" => Ok(SecModOps::NAME),
        "sh_type" => Ok(SecModOps::TYPE),
        "sh_flags" => Ok(SecModOps::FLAGS),
        "sh_addr" => Ok(SecModOps::ADDR),
        "sh_offset" => Ok(SecModOps::OFFSET),
        "sh_size" => Ok(SecModOps::SIZE),
        "sh_link" => Ok(SecModOps::LINK),
        "sh_info" => Ok(SecModOps::INFO),
        "sh_addralign" => Ok(SecModOps::ADDRALIGN),
        "sh_entsize" => Ok(SecModOps::ENTSIZE),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            ("Invalid modify option provided for section header fields")))
    }
}

pub fn get_sec_field(option: SecModOps) -> String {
    match option {
        SecModOps::NAME => {
            "sh_name".to_string()
        }
        SecModOps::TYPE => {
            "sh_type".to_string()
        }
        SecModOps::FLAGS => {
            "sh_flags".to_string()
        }
        SecModOps::ADDR => {
            "sh_addr".to_string()
        }
        SecModOps::OFFSET => {
            "sh_offset".to_string()
        }
        SecModOps::SIZE => {
            "sh_size".to_string()
        }
        SecModOps::LINK => {
            "sh_link".to_string()
        }
        SecModOps::INFO => {
            "sh_info".to_string()
        }
        SecModOps::ADDRALIGN => {
            "sh_addralign".to_string()
        }
        SecModOps::ENTSIZE => {
            "sh_entsize".to_string()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SegModOps {
    TYPE,
    OFFSET,
    VADDR,
    PADDR,
    FILESZ,
    MEMSZ,
    FLAGS,
    ALIGN,
}

pub fn parse_seg_mod_ops(option: String) -> Result<SegModOps, std::io::Error> {
    match option.as_str() {
        "p_type" => Ok(SegModOps::TYPE),
        "p_offset" => Ok(SegModOps::OFFSET),
        "p_vaddr" => Ok(SegModOps::VADDR),
        "p_paddr" => Ok(SegModOps::PADDR),
        "p_filesz" => Ok(SegModOps::FILESZ),
        "p_memsz" => Ok(SegModOps::MEMSZ),
        "p_flags" => Ok(SegModOps::FLAGS),
        "p_align" => Ok(SegModOps::ALIGN),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            ("Invalid modify option provided for program header fields")))
    }
}


pub fn get_seg_field(option: SegModOps) -> String {
    match option {
        SegModOps::TYPE => {
            "p_type".to_string()
        }
        SegModOps::OFFSET => {
            "p_offset".to_string()
        }
        SegModOps::VADDR => {
            "p_vaddr".to_string()
        }
        SegModOps::PADDR => {
            "p_paddr".to_string()
        }
        SegModOps::FILESZ => {
            "p_filesz".to_string()
        }
        SegModOps::MEMSZ => {
            "p_memsz".to_string()
        }
        SegModOps::FLAGS => {
            "p_flags".to_string()
        }
        SegModOps::ALIGN => {
            "p_align".to_string()
        }
    }
}

