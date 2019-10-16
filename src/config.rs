use std::env;
use std::process::exit;

/* Default injection mode values */
const INJ_DEFAULT_SIZE: usize = 0x1000;
const INJ_DEFAULT_EXT: &'static str= ".text";
const INJ_DEFAULT_ENTRY: Option<usize> = None;

/*
* Parse args such that a user can enter args in any order. All args other than input file
* and modification type are optional for injection mode; for modification mode, all args are
* required.
*
* TODO: implement support for config file parsing
*/
pub fn parse_args(infile: &mut String,
              outfile: &mut String,
              options: &mut DedElfOps) {
    //shorthand mode
    let s_i_mode = "-i";
    let s_m_mode = "-m";
    //longhand mode
    let l_i_mode = "--inject";
    let l_m_mode = "--modify";

    let input_mode = env::args().nth(1).unwrap();

    *infile = env::args().nth(2).unwrap();
    let mut file = env::args().nth(2).unwrap();

    let temp = "_inj".to_string();
    //default outfile name is _inj appended to original file name
    file.push_str(&temp);
    *outfile = file.clone();

    let num_args = env::args().len();

    match input_mode.as_str() {

        _ if s_i_mode == input_mode || l_i_mode == input_mode => {
            println!("Injection mode!");
            let inj: String = env::args().nth(3).unwrap();

            //only even parity number of args are acceptable for injection mode
            if num_args % 2 != 0 {
                super::display_useage();
                exit(1);
            } else if num_args != 4 {

                println!("More than 4!");
                let mut flag = false;
                let mut args = Vec::new();
                for i in 4..num_args {
                    let prev_arg = env::args().nth(i - 1).unwrap();
                    let arg = env::args().nth(i).unwrap();
                    println!("Prev arg is {:?}, arg is {:?}", prev_arg, arg);
                    //if option to specify outfile name, reset the default
                    //and dont push these values as they are not mode configuration
                    //vals
                    if prev_arg.as_str() == "-o" {
                        *outfile = arg;
                        flag = true;
                        continue;
                    } else if arg.as_str() == "-o" {
                        continue;
                    } else {
                        println!("Pushing! arg {:?}", arg);
                        args.push(arg);
                    }
                }
                if args.len() == 0 && flag {

                    *options = DedElfOps::default_inj_ops(inj);
                } else if args.len() == 0 {
                    super::display_useage();
                    exit(1);
                } else {
                    let res = DedElfOps::parse_inj_ops(args, inj);
                    if res.is_err() {
                        super::display_useage();
                        exit(1);
                    }
                    *options = res.unwrap();
                }
            }
            //return config::Mode::INJECT
        }
        _ if s_m_mode == input_mode || l_m_mode == input_mode => {
            println!("Mod mode!");
            let exec = "exec_header";
            let sec = "sec_header";
            let seg = "prog_header";
            let mod_type = env::args().nth(3).unwrap();
            let mut args = Vec::new();
            for i in 4..num_args {
                let arg = env::args().nth(i).unwrap();
                if arg.as_str() == "-o" {
                    break;
                } else {
                    println!("Pushing! arg {:?}", arg);
                    args.push(arg);
                }
            }

            let res = match mod_type {
                _ if mod_type.as_str() == exec => {
                    println!("Mod mode! exec header!");

                    match num_args {
                        6 => {},
                        8 => {
                            if env::args().nth(6).unwrap().as_str() != "-o" {
                                super::display_useage();
                                exit(1);
                            }
                            *outfile = env::args().nth(7).unwrap();

                        },
                        _ => {
                            super::display_useage();
                            exit(1);
                        }
                    }
                    DedElfOps::parse_mod_ops(ModOps::EXEC, args)

                },

                _ if mod_type.as_str() == sec => {
                    println!("Mod mode! sec header!");

                    match num_args {
                        7 => {},
                        9 => {
                            if env::args().nth(7).unwrap().as_str() != "-o" {
                                super::display_useage();
                                exit(1);
                            }
                            *outfile = env::args().nth(8).unwrap();
                        },
                        _ => {
                            super::display_useage();
                            exit(1);
                        }
                    }
                    DedElfOps::parse_mod_ops(ModOps::SECTION, args)
                },
                _ if mod_type.as_str() == seg => {
                    println!("Mod mode! seg header!");

                    match num_args {
                        7 => {

                        },
                        9 => {
                            if env::args().nth(7).unwrap().as_str() != "-o" {
                                super::display_useage();
                                exit(1);
                            }
                            *outfile = env::args().nth(8).unwrap();
                        },
                        _ => {
                            super::display_useage();
                            exit(1);
                        }
                    }
                    DedElfOps::parse_mod_ops(ModOps::SEGMENT, args)
                },
                _ => {
                    super::display_useage();
                    exit(1);
                    Err(())
                }
            };
            if res.is_err(){
                super::display_useage();
                exit(1); //todo hardcode error return vals?
            }
            *options = res.unwrap();
        }
        _ => {
            super::display_useage();
            exit(1); //todo hardcode error return vals?
        }
    }
   // return config::Mode::INJECT
}


#[derive(Debug)]
pub struct DedElfOps {
    pub injection: Option<InjModeOps>,
    pub modify: Option<ModModeOps>,
}

impl DedElfOps {
    pub fn parse_inj_ops(user_input: Vec<String>, file: String) -> Result<DedElfOps, ()> {
        let s = "-s";
        let p = "-p";
        let e = "-e";
        let mut size: Option<usize> = None;
        let mut entry: Option<usize> = None;
        let mut extend: Option<String> = None;
        let def_ops: InjOps = match user_input.len() {

            2 => {
                println!("Case 2!");

                match user_input[0] {
                    _ if user_input[0].as_str() == s => {
                        println!("Case 2.1!");
                        let trimmed = user_input[1].trim_start_matches("0x");

                        let check = usize::from_str_radix(&trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }

                        size = Some(check.unwrap());
                        InjOps::SIZE
                    },
                    _ if user_input[0].as_str() == p => {
                        println!("Case 2.2!");

                        extend = Some(user_input[1].clone());
                        InjOps::EXTEND
                    },
                    _ if user_input[0].as_str() == e => {
                        println!("Case 2.3!");
                        let trimmed = user_input[1].trim_start_matches("0x");

                        let check = usize::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }

                        entry = Some(check.unwrap());
                        InjOps::CHANGE_ENTRY
                    },
                    _ => { return Err(()) }
                }
            },
            4 => {
                println!("Case 4!");

                match user_input[0]{//}, user_input[2]) {

                    _ if (user_input[0].as_str() == s) &&
                        (user_input[2].as_str() == p) => {
                        println!("Case 4.1!");
                        let trimmed = user_input[1].trim_start_matches("0x");
                        let check = usize::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }
                        size = Some(check.unwrap());
                        extend = Some(user_input[3].clone());
                        InjOps::NO_ENTRY
                    },
                    _ if (user_input[0].as_str() == p) &&
                        (user_input[2].as_str() == e) => {
                        println!("Case 4.2!");

                        extend = Some(user_input[1].clone());
                        let trimmed = user_input[3].trim_start_matches("0x");
                        let check = usize::from_str_radix(&trimmed, 16);
                        if check.is_err() {
                            println!("Its bad-- {:?}", user_input[3],);
                            return Err(());
                        }
                        entry = Some(check.unwrap());
                        InjOps::NO_SIZE
                    },
                    _ if (user_input[0].as_str() == s) &&
                        (user_input[2].as_str() == e) => {
                        println!("Case 4.3!");
                        let trimmed = user_input[1].trim_start_matches("0x");
                        let check = usize::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }
                        size = Some(check.unwrap());
                        let trimmed = user_input[3].trim_start_matches("0x");
                        let check = usize::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }
                        entry = Some(check.unwrap());
                        InjOps::NO_EXTEND
                    },

                    _ if (user_input[2].as_str() == s) &&
                        (user_input[0].as_str() == p) => {
                        println!("Case 4.4!");
                        let trimmed = user_input[3].trim_start_matches("0x");
                        let check = usize::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }
                        size = Some(check.unwrap());
                        extend = Some(user_input[1].clone());
                        InjOps::NO_ENTRY
                    },
                    _ if (user_input[2].as_str() == p) &&
                        (user_input[0].as_str() == e) => {
                        println!("Case 4.5!");

                        extend = Some(user_input[3].clone());
                        let trimmed = user_input[1].trim_start_matches("0x");
                        let check = usize::from_str_radix(&trimmed, 16);
                        if check.is_err() {
                            println!("Its bad-- {:?}", user_input[3],);
                            return Err(());
                        }
                        entry = Some(check.unwrap());
                        InjOps::NO_SIZE
                    },
                    _ if (user_input[2].as_str() == s) &&
                        (user_input[0].as_str() == e) => {
                        println!("Case 4.6!");
                        let trimmed = user_input[3].trim_start_matches("0x");
                        let check = usize::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }
                        size = Some(check.unwrap());
                        let trimmed = user_input[1].trim_start_matches("0x");
                        let check = usize::from_str_radix(trimmed, 16);
                        if check.is_err() {
                            return Err(());
                        }
                        entry = Some(check.unwrap());
                        InjOps::NO_EXTEND
                    },
                    _ => {
                        println!("Womp womp...val is {:?} {:?}", user_input[0], user_input[2]);
                        return Err(())
                    }
                }
            },
            6 => {
                println!("Case 6!");

                let mut temp_size: Option<String> = None;
                let mut temp_entry: Option<String> = None;

                match user_input[0]{//} | user_input[2]| user_input[4]) {
                    _ if (user_input[0].as_str() == s) &&
                        (user_input[2].as_str() == p) &&
                        (user_input[4].as_str() == e) => {
                        println!("This case!! 6.1");

                        temp_size = Some(user_input[1].clone());
                        temp_entry = Some(user_input[5].clone());
                        extend = Some(user_input[3].clone());

                        // InjOps::ALL
                    },
                    _ if (user_input[0].as_str() == s) &&
                        (user_input[2].as_str() == e) &&
                        (user_input[4].as_str() == p) => {
                        println!("This case!! 6.2");

                        temp_size = Some(user_input[1].clone());
                        temp_entry = Some(user_input[3].clone());
                        extend = Some(user_input[5].clone());

                        // InjOps::ALL
                    },
                    _ if (user_input[0].as_str() == e) &&
                        (user_input[2].as_str() == p) &&
                            (user_input[4].as_str() == s) => {
                        println!("This case!! 6.3");

                        temp_size = Some(user_input[5].clone());
                        temp_entry = Some(user_input[1].clone());
                        extend = Some(user_input[3].clone());

                        //InjOps::ALL
                    },
                    _ if (user_input[0].as_str() == e)&&
                        (user_input[2].as_str() == s)&&
                        (user_input[4].as_str() == p) => {
                        println!("This case!!6.4");

                        temp_size = Some(user_input[3].clone());
                        temp_entry = Some(user_input[1].clone());
                        extend = Some(user_input[5].clone());

                        //InjOps::ALL
                    },
                    _ if (user_input[0].as_str() == p)&&
                        (user_input[2].as_str() == s)&&
                        (user_input[4].as_str() == e) => {
                        println!("This case!! 6.5");
                        temp_size = Some(user_input[3].clone());
                        temp_entry = Some(user_input[1].clone());
                        extend = Some(user_input[5].clone());

                        //InjOps::ALL
                    },
                    _ if (user_input[0].as_str() == p)&&
                        (user_input[2].as_str() == e)&&
                            (user_input[4].as_str() == s) => {
                        println!("This case!! 6.7");

                        temp_size = Some(user_input[5].clone());
                        temp_entry = Some(user_input[3].clone());
                        extend = Some(user_input[1].clone());

                        //InjOps::ALL
                    },
                    _ => { println!("Bad parsed config options"); return Err(()) }
                };

                let temp = temp_size.unwrap().clone();
                let trimmed = temp.trim_start_matches("0x");

                let check = usize::from_str_radix(trimmed, 16);
                if check.is_err() {
                    println!("Size is wrong: {:?}", temp);
                    return Err(());
                }

                size = Some(check.unwrap());
                let temp = temp_entry.unwrap().clone();
                let trimmed = temp.trim_start_matches("0x");
                let check = usize::from_str_radix(trimmed, 16);
                if check.is_err() {
                    println!("entry is wrong {:?}", temp);
                    return Err(());
                }

                entry = Some(check.unwrap());
                InjOps::ALL
            },
            _ => { return Err(()) }
        };
        let ops = InjModeOps::parse_inj_ops(def_ops, size, extend, entry, file);
        if ops.is_err() {
            return Err(())
        }
        Ok(DedElfOps { injection: Some(ops.unwrap()), modify: None })
    }

    pub fn no_ops()->DedElfOps{
        DedElfOps{
            injection: None,
            modify: None,
        }
    }
    pub fn default_inj_ops(file: String) -> DedElfOps {
        DedElfOps {
            injection: Some(
                InjModeOps::default(file)
            ),
            modify: None,
        }
    }

    pub fn parse_mod_ops(op: ModOps, user_input: Vec<String>)->Result<DedElfOps,()> {
        println!("More parsing of mod ops!");
        let len = user_input.len();
        let mod_ops = match op {
            ModOps::EXEC => {
                if len != 2 {
                    return Err(())
                }

                let op = parse_exec_mod_ops(user_input[0].clone());
                if op.is_err() {
                    println!("Exec mode op error!");
                    return Err(());
                }
                ModModeOps {
                    exec: Some(ExecCfg {
                        op_mode: op.unwrap(),
                        replacement: user_input[1].clone(),
                    }),
                    sec: None,
                    seg: None,
                }
            }
            ModOps::SECTION => {
                if len != 3 {
                    return Err(())
                }
                let op = parse_sec_mod_ops(user_input[1].clone());
                if op.is_err() {
                    println!("Sec mode op error!");

                    return Err(());
                }
                let cfg: SecCfg;
                let trimmed = user_input[0].trim_start_matches("0x");
                let check = usize::from_str_radix(trimmed.clone(), 16);
                if check.is_err() {
                    cfg = SecCfg {
                        op_mode: op.unwrap(),
                        sec_name: Some(user_input[0].clone()),
                        sec_idx: None,
                        replacement: user_input[2].clone(),
                    };
                } else {
                    cfg = SecCfg {
                        op_mode: op.unwrap(),
                        sec_name: None,
                        sec_idx: Some(check.unwrap()),
                        replacement: user_input[2].clone(),
                    };
                }

                ModModeOps {
                    exec: None,
                    sec: Some(cfg),
                    seg: None,
                }
            }
            ModOps::SEGMENT => {
                let op = parse_seg_mod_ops(user_input[1].clone());
                if op.is_err() {
                    println!("Seg mode op error!");

                    return Err(());
                }

                let trimmed = user_input[0].trim_start_matches("0x");
                let check = usize::from_str_radix(trimmed, 16);
                if check.is_err() {
                    println!("Seg mode op error v2!");

                    return Err(());
                }
                ModModeOps {
                    exec: None,
                    sec: None,
                    seg: Some(SegCfg {
                        op_mode: op.unwrap(),
                        seg_idx: check.unwrap(),
                        replacement: user_input[2].clone(),
                    }),
                }
            }
        };

        println!("Returning some mod ops! {:?}", mod_ops);
        Ok(DedElfOps {
            injection: None,
            modify: Some(mod_ops),
        })
    }

    pub fn get_inj_file(&self)->Option<String>{
        if let Some(inj) = &self.injection {
            return Some(inj.file.clone())
        } else {
            return None
        }
    }

}

pub enum Mode {
    MODIFY,
    INJECT,
    DUAL,
}
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum InjOps {
    DEFAULT,
    SIZE,
    EXTEND,
    CHANGE_ENTRY,
    ALL,
    NO_EXTEND,
    NO_ENTRY,
    NO_SIZE,
}

#[derive(Clone, Debug)]
pub struct InjModeOps {
    file: String,
    size: usize,
    extend: String,
    new_entry: Option<usize>,
}

impl InjModeOps {
    pub fn parse_inj_ops(op_type: InjOps,
                        size: Option<usize>,
                         extend: Option<String>,
                         entry: Option<usize>,
                            file: String) -> Result<InjModeOps,()> {

        let mut new_size: usize = INJ_DEFAULT_SIZE;
        let mut new_entry: Option<usize> = None;
        let mut new_extend: String = INJ_DEFAULT_EXT.to_string();

        if let Some(size)=size{
            new_size = size;
        }
        if let Some(entry)=entry{
            new_entry = Some(entry);
        }
        if let Some(extend) = extend{
            new_extend = extend;
        }

        match op_type {
            InjOps::DEFAULT => {
               Ok(InjModeOps::default(file))
            },
            InjOps::SIZE => {
                Ok(InjModeOps{
                    file: file,
                    size: new_size,
                    extend: INJ_DEFAULT_EXT.to_string(),
                    new_entry: None,
                })
            },
            InjOps::EXTEND => {
                Ok(InjModeOps{
                    file: file,
                    size: INJ_DEFAULT_SIZE,
                    extend: new_extend,
                    new_entry: None,
                })
            },
            InjOps::CHANGE_ENTRY => {
                Ok(InjModeOps{
                    file: file,
                    size: INJ_DEFAULT_SIZE,
                    extend: INJ_DEFAULT_EXT.to_string(),
                    new_entry: new_entry,
                })
            },
            InjOps::ALL => {
                Ok(InjModeOps{
                    file: file,
                    size: new_size,
                    extend: new_extend,
                    new_entry: new_entry,
                })
            },
            InjOps::NO_EXTEND => {
                Ok(InjModeOps{
                    file: file,
                    size: new_size,
                    extend: INJ_DEFAULT_EXT.to_string(),
                    new_entry: new_entry,
                })
            },
            InjOps::NO_ENTRY => {
                Ok(InjModeOps{
                    file: file,
                    size: new_size,
                    extend: new_extend,
                    new_entry: None,
                })
            },
            InjOps::NO_SIZE => {
                Ok(InjModeOps{
                    file: file,
                    size: INJ_DEFAULT_SIZE,
                    extend: new_extend,
                    new_entry: new_entry,
                })
            },
        }
    }

    fn default(file: String)-> InjModeOps{
        InjModeOps{
            file: file,
            size: INJ_DEFAULT_SIZE,
            extend: INJ_DEFAULT_EXT.to_string(),
            new_entry: INJ_DEFAULT_ENTRY,
        }
    }

    pub fn get_size(&self)->usize{
        self.size
    }

    pub fn get_extend(&self)->String{
        self.extend.clone()
    }

    pub fn get_file(&self)->String{
        self.file.clone()
    }

    pub fn get_entry(&self)->Option<usize>{
        self.new_entry
    }

}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum ModOps {
    EXEC,
    SECTION,
    SEGMENT,
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

#[derive(Clone,Debug)]
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

pub fn parse_exec_mod_ops(option: String) -> Result<ExecModOps, ()> {
    match option.as_str() {
        "e_ident" => Ok(ExecModOps::IDENT),
       "EI_CLASS" => Ok(ExecModOps::IDENT),
       "EI_DATA"=> Ok(ExecModOps::IDENT),
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
        _ => Err(()),
    }
}

pub fn get_exec_field(option: ExecModOps)->String{
    match option {
            ExecModOps::IDENT =>{
                "e_ident".to_string()
            }
        /*
        ExecModOps::CLASS=> {
            "EI_CLASS".to_string()
        }
        ExecModOps::DATA=> {
            "EI_DATA".to_string()
        }
        ExecModOps::OSABI=> {
            "EI_OSABI".to_string()
        }*/

            ExecModOps::TYPE =>{
                "e_type".to_string()
            }
            ExecModOps::MACH =>{
                "e_machine".to_string()
            }
            ExecModOps::VERSION =>{
                "e_version".to_string()
            }
            ExecModOps::ENTRY =>{
                "e_entry".to_string()
            }
            ExecModOps::PHOFF =>{
                "e_phoff".to_string()
            }
            ExecModOps::SHOFF =>{
                "e_shoff".to_string()
            }
            ExecModOps::FLAGS =>{
                "e_flags".to_string()
            }
            ExecModOps::EHSIZE =>{
                "e_ehsize".to_string()
            }
            ExecModOps::PHENTSIZE =>{
                "e_phentsize".to_string()
            }
            ExecModOps::PHNUM =>{
                "e_phnum".to_string()
            }
            ExecModOps::SHENTSIZE =>{
                "e_shentsize".to_string()
            }
            ExecModOps::SHNUM =>{
                "e_shnum".to_string()
            }
            ExecModOps::SHSTRNDX =>{
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

pub fn parse_sec_mod_ops(option: String) -> Result<SecModOps, ()> {
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
        _ => Err(()),
    }
}

pub fn get_sec_field(option: SecModOps)->String{
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

pub fn parse_seg_mod_ops(option: String) -> Result<SegModOps, ()> {
    match option.as_str() {
        "p_type" => Ok(SegModOps::TYPE),
        "p_offset" => Ok(SegModOps::OFFSET),
        "p_vaddr" => Ok(SegModOps::VADDR),
        "p_paddr" => Ok(SegModOps::PADDR),
        "p_filesz" => Ok(SegModOps::FILESZ),
        "p_memsz" => Ok(SegModOps::MEMSZ),
        "p_flags" => Ok(SegModOps::FLAGS),
        "p_align" => Ok(SegModOps::ALIGN),
        _ => Err(()),
    }
}


pub fn get_seg_field(option: SegModOps) -> String {
    match option {
        SegModOps::TYPE =>{
            "p_type".to_string()
        }
        SegModOps::OFFSET =>{
            "p_offset".to_string()
        }
         SegModOps::VADDR =>{
             "p_vaddr".to_string()
        }
        SegModOps::PADDR=>{
            "p_paddr".to_string()
        }
        SegModOps::FILESZ =>{
            "p_filesz".to_string()
        }
        SegModOps::MEMSZ =>{
            "p_memsz".to_string()
        }
        SegModOps::FLAGS=> {
            "p_flags".to_string()
        }
        SegModOps::ALIGN => {
            "p_align".to_string()
        }
    }
}

