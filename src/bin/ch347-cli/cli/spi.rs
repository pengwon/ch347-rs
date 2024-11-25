use clap::{Arg, Command};

pub fn spi_subcommand() -> Command {
    Command::new("spi")
        .about("SPI subcommands")
        .subcommand(
            Command::new("init")
                .about("Initialize SPI")
                .arg(
                    Arg::new("fd")
                        .required(false)
                        .long("device-index")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u32))
                        .help("Device index"),
                )
                .arg(
                    Arg::new("mode")
                        .required(false)
                        .long("mode")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u8))
                        .help("SPI mode, 0-3: SPI Mode0/1/2/3"),
                )
                .arg(
                    Arg::new("clock")
                        .required(false)
                        .long("clock")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u8))
                        .help("SPI clock, 0=60MHz, 1=30MHz, 2=15MHz, 3=7.5MHz, 4=3.75MHz, 5=1.875MHz, 6=937.5KHz, 7=468.75KHz"),
                )
                .arg(
                    Arg::new("byte_order")
                        .required(false)
                        .long("byte-order")
                        .default_value("1")
                        .value_parser(clap::value_parser!(u8))
                        .help("Byte order, 0=LSB, 1=MSB"),
                )
                .arg(
                    Arg::new("spi_write_read_interval")
                        .required(false)
                        .long("write-read-interval")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u16))
                        .help("SPI write-read interval in microseconds"),
                )
                .arg(
                    Arg::new("spi_out_default_data")
                        .required(false)
                        .long("default-data")
                        .default_value("255") // 0xff in decimal
                        .value_parser(clap::value_parser!(u8))
                        .help("Default output data when SPI reads data"),
                )
                .arg(
                    Arg::new("chip_select")
                        .required(false)
                        .long("chip-select")
                        .default_value("128") // 0x80 in decimal
                        .value_parser(clap::value_parser!(u32))
                        .help("Chip select control, Bit7=0: ignore chip select control; Bit7=1: parameter is valid, Bit1/Bit0 are 00/01 to select the CS1/CS2 pins as low-level active chip select, respectively"),
                )
                .arg(
                    Arg::new("cs1_polarity")
                        .required(false)
                        .long("cs1-polarity")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u8))
                        .help("CS1 polarity control: 0=active low, 1=active high"),
                )
                .arg(
                    Arg::new("cs2_polarity")
                        .required(false)
                        .long("cs2-polarity")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u8))
                        .help("CS2 polarity control: 0=active low, 1=active high"),
                )
                .arg(
                    Arg::new("is_auto_deactive_cs")
                        .required(false)
                        .long("auto-deactive-cs")
                        .default_value("1")
                        .value_parser(clap::value_parser!(u16))
                        .help("Auto undo chip selection or not after operation is completed"),
                )
                .arg(
                    Arg::new("active_delay")
                        .required(false)
                        .long("active-delay")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u16))
                        .help("Setting delay time for performing read/write operations after chip selection, in microseconds"),
                )
                .arg(
                    Arg::new("delay_deactive")
                        .required(false)
                        .long("delay-deactive")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u32))
                        .help("Delay time for read/write operations after de-selecting chip selection, in microseconds"),
                ),
        )
        .subcommand(
            Command::new("get-cfg")
                .arg(
                    Arg::new("fd")
                        .required(false)
                        .default_value("0")
                        .value_parser(clap::value_parser!(u32))
                        .help("Device index"),
                )
                .about("Get SPI configuration"),
        )
        .subcommand(
            Command::new("write")
                .about("Write data to SPI")
                .arg(
                    Arg::new("fd")
                        .required(false)
                        .long("device-index")
                        .default_value("0")
                        .value_parser(clap::value_parser!(u32))
                        .help("Device index"),
                )
                .arg(
                    Arg::new("chip_select")
                        .required(false)
                        .long("chip-select")
                        .default_value("1")
                        .value_parser(clap::value_parser!(u32))
                        .help("Chip select, 0 for INACTIVE, 1 for CS1, 2 for CS2"),
                )
                .arg(
                    Arg::new("write_step")
                        .required(false)
                        .long("write-step")
                        .default_value("512")
                        .value_parser(clap::value_parser!(u32))
                        .help("The length of a single block to be read"),
                )
                .arg(
                    Arg::new("data")
                        .required(true)
                        .long("data")
                        .value_parser(clap::value_parser!(String))
                        .help("Data to write"),
                ),
        )
        .subcommand(
            Command::new("read").about("Read from SPI").arg(
                Arg::new("length")
                    .required(false)
                    .default_value("1")
                    .value_parser(clap::value_parser!(u32))
                    .help("Length"),
            ),
        )
        .subcommand(Command::new("write-read").about("Write then read from SPI"))
}
