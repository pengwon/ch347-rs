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
                    Arg::new("dir-out")
                        .required(true)
                        .long("dir-out")
                        .value_parser(clap::value_parser!(String))
                        .help("Set GPIO direction (0: input, 1: output)"),
                )
                .arg(
                    Arg::new("data-out")
                        .required(true)
                        .long("data-out")
                        .value_parser(clap::value_parser!(String))
                        .help("Set GPIO data (0: low, 1: high)"),
                ),
        )
        .subcommand(
            Command::new("pwm")
                .about("Set PWM")
                .arg(
                    Arg::new("fd")
                        .required(false)
                        .long("device-index")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u32))
                        .help("Device index"),
                )
                .arg(
                    Arg::new("channel")
                        .required(true)
                        .long("channel")
                        .value_parser(clap::value_parser!(String))
                        .help("PWM channels"),
                )
                .arg(
                    Arg::new("frequency")
                        .required(true)
                        .long("frequency")
                        .value_parser(clap::value_parser!(u32))
                        .help("PWM frequency in Hz"),
                )
                .arg(
                    Arg::new("duty_cycle")
                        .required(true)
                        .long("duty-cycle")
                        .value_parser(clap::value_parser!(String))
                        .help("PWM duty cycle in percentage (0-100)"),
                )
                .arg(
                    Arg::new("pulse_count")
                        .required(true)
                        .long("pulse-count")
                        .value_parser(clap::value_parser!(u32))
                        .help("Number of pulses (0 for continuous output)"),
                ),
        )
}
