use clap::{Arg, Command};
mod spi;

pub fn build_cli() -> Command {
    Command::new("ch347-cli")
        .version("v0.1.0")
        .author("Peter <peter@boringhex.top>")
        .about("CLI for CH347")
        .subcommand(Command::new("list").about("List all devices"))
        .subcommand(
            Command::new("close")
                .about("Close device")
                .arg(
                    Arg::new("all")
                        .long("all")
                        .short('A')
                        .help("Close all devices")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("fd")
                        .required(false)
                        .default_value("0")
                        .value_parser(clap::value_parser!(u32))
                        .help("Device index"),
                ),
        )
        .subcommand(spi::spi_subcommand())
}
