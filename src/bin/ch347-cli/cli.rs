use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("ch347-cli")
        .version("v0.1.0")
        .author("Peter <peter@boringhex.top>")
        .about("CLI for CH347")
        .subcommand(Command::new("list").about("List all devices"))
        .subcommand(
            Command::new("spi").about("SPI commands").subcommand(
                Command::new("init")
                    .about("Initialize SPI")
                    .arg(
                        Arg::new("fd")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u32))
                            .help("Device index"),
                    )
                    .arg(
                        Arg::new("mode")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u8))
                            .help("SPI mode"),
                    )
                    .arg(
                        Arg::new("clock")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u8))
                            .help("SPI clock"),
                    )
                    .arg(
                        Arg::new("byte_order")
                            .required(false)
                            .default_value("1")
                            .value_parser(clap::value_parser!(u8))
                            .help("Byte order"),
                    )
                    .arg(
                        Arg::new("spi_write_read_interval")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u16))
                            .help("SPI write-read interval"),
                    )
                    .arg(
                        Arg::new("spi_out_default_data")
                            .required(false)
                            .default_value("255") // 0xff in decimal
                            .value_parser(clap::value_parser!(u8))
                            .help("SPI out default data"),
                    )
                    .arg(
                        Arg::new("chip_select")
                            .required(false)
                            .default_value("128") // 0x80 in decimal
                            .value_parser(clap::value_parser!(u32))
                            .help("Chip select"),
                    )
                    .arg(
                        Arg::new("cs1_polarity")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u8))
                            .help("CS1 polarity"),
                    )
                    .arg(
                        Arg::new("cs2_polarity")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u8))
                            .help("CS2 polarity"),
                    )
                    .arg(
                        Arg::new("is_auto_deactive_cs")
                            .required(false)
                            .default_value("1")
                            .value_parser(clap::value_parser!(u16))
                            .help("Is auto deactive CS"),
                    )
                    .arg(
                        Arg::new("active_delay")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u16))
                            .help("Active delay"),
                    )
                    .arg(
                        Arg::new("delay_deactive")
                            .required(false)
                            .default_value("0")
                            .value_parser(clap::value_parser!(u32))
                            .help("Delay deactive"),
                    ),
            ),
        )
}
