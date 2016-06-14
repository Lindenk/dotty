use clap::ArgMatches;

use subcommands::Command;
use subcommands::install::InstallOptions;

pub fn parse_cli_args(args : ArgMatches) -> Command {
    match args.subcommand_name() {
        Some("install") => {
            Command::Install(InstallOptions{})
        },
        _ => unimplemented!()
    }
}