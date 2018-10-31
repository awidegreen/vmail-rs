use clap::{App, AppSettings};

use cmd::alias;
use cmd::domain;
use cmd::user;

pub fn build_cli() -> App<'static, 'static> {
    App::new("vmail")
        .version("0.1.0")
        .author("Armin Widegreen <vmail@widegreen.net>")
        .about("A CLI client for vmail")
        .global_setting(AppSettings::InferSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(user::get_subcommand())
        .subcommand(domain::get_subcommand())
        .subcommand(alias::get_subcommand())
}
