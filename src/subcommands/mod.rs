pub mod install;
pub mod remove;
pub mod update;
pub mod reinstall;

use error;
use config::Config;
use self::install::{install, InstallOptions};
use self::remove::{remove, RemoveOptions};
use self::update::{update, UpdateOptions};
use self::reinstall::{reinstall, ReinstallOptions};

pub enum Command {
    Install(InstallOptions),
    Remove(RemoveOptions),
    Update(UpdateOptions),
    Reinstall(ReinstallOptions),
}

impl Command {
    pub fn run(&self, conf : &Config) -> Result<(), error::DottyError> {
        match *self {
            Command::Install(ref opts) => install(opts, conf),
            Command::Remove(ref opts) => remove(opts, conf),
            Command::Update(ref opts) => update(opts, conf),
            Command::Reinstall(ref opts) => reinstall(opts, conf),
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