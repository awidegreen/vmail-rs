use clap::{App, AppSettings, Arg, SubCommand};

use cmd::alias;
use cmd::domain;
use cmd::user;

pub fn build_cli() -> App<'static, 'static> {
    App::new("vmail-cli")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .global_setting(AppSettings::InferSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(user::get_subcommand())
        .subcommand(domain::get_subcommand())
        .subcommand(alias::get_subcommand())
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generates completion scripts for your shell")
                .setting(AppSettings::Hidden)
                .arg(
                    Arg::with_name("SHELL")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh"])
                        .help("The shell to generate the script for"),
                ),
        )
}
