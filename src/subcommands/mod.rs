pub mod install;

use error;
use self::install::{install, InstallOptions};

pub enum Command {
    Install(InstallOptions),
}

impl Command {
    pub fn run(&self) -> Result<(), error::DottyError> {
        match *self {
            Command::Install(ref opts) => install(opts)
        }
        
            //let get_module_name = |m| cli_args.subcommand_matches(m).unwrap().value_of("module_name");

/*
    match mod_name {
        Some(m @ "install") => {
            let try!(module = ModuleData::load(get_module_name(m).unwrap()));
            try!(install(module));
        },
        Some(m @ "remove") => {
            unimplemented!();
        },
        Some(m @ "reinstall") => {
            unimplemented!();
        },
        Some(m @ "update") => {
            if let Some(m_name) = get_module_name(m) {
                unimplemented!();
            } else {
                unimplemented!();
            }
        },
        _ => unreachable!()
    };
 */   
    }
}