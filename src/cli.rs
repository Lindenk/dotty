use clap::ArgMatches;
use std::path::{PathBuf, Path};

use subcommands::Command;
use subcommands::install::InstallOptions;
use subcommands::remove::RemoveOptions;
use subcommands::update::UpdateOptions;
use subcommands::reinstall::ReinstallOptions;

/// This function assumes a subcommand exists
fn get_sub_opt(args : &ArgMatches, opt : &str) -> Option<String> {
    args.subcommand_matches(
        args.subcommand_name().unwrap()
    ).unwrap().value_of(opt).map(String::from)
}

pub fn parse_cli_args(args : ArgMatches) -> Command {
    match args.subcommand_name() {
        Some("install") => {
            Command::Install(InstallOptions{
                // Come on rust >.>, this parses the filename off of the path given
                module_name: String::from(Path::new(get_sub_opt(&args, "module_name").unwrap().as_str()).file_name().unwrap_or_default().to_str().unwrap()),
                module_path: PathBuf::from(get_sub_opt(&args, "module_name").unwrap())
            })
        },
        Some("remove") => {
            Command::Remove(RemoveOptions{
                module_name: get_sub_opt(&args, "module_name").unwrap()
            })
        },
        Some("update") => {
            Command::Update(UpdateOptions{
                module_name: get_sub_opt(&args, "module_name")
            })
        },
        Some("reinstall") => {
            Command::Reinstall(ReinstallOptions{
                module_name: get_sub_opt(&args, "module_name")
            })
        }
        _ => unreachable!()
    }
}
