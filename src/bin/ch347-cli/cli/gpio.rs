use clap::{Arg, Command};

pub fn gpio_subcommand() -> Command {
    Command::new("gpio")
        .about("GPIO subcommands")
        .subcommand(
            Command::new("get").about("Get GPIO").arg(
                Arg::new("fd")
                    .required(false)
                    .long("device-index")
                    .default_value("0")
                    .value_parser(clap::value_parser!(u32))
                    .help("Device index"),
            ),
        )
        .subcommand(
            Command::new("set")
                .about("Set GPIO")
                .arg(
                    Arg::new("fd")
                        .required(false)
                        .long("device-index")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u32))
                        .help("Device index"),
                )
                .arg(
                    Arg::new("enable")
                        .required(true)
                        .long("enable")
                        .value_parser(clap::value_parser!(String))
                        .help("Enable GPIO"),
                )
                .arg(
                    Arg::new("set-dir-out")
                        .required(true)
                        .long("dir-out")
                        .value_parser(clap::value_parser!(String))
                        .help("Set GPIO direction (0: input, 1: output)"),
                )
                .arg(
                    Arg::new("set-data-out")
                        .required(true)
                        .long("data-out")
                        .value_parser(clap::value_parser!(String))
                        .help("Set GPIO data (0: low, 1: high)"),
                ),
        )
}
