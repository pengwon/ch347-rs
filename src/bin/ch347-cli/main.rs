use std::process;

mod cli;
mod cmd;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => cmd::list_devices(),
        Some(("close", sub_matches)) => cmd::close_devices(sub_matches),
        Some(("spi", sub_matches)) => match sub_matches.subcommand() {
            Some(("init", init_matches)) => cmd::cmd_spi_init(init_matches),
            Some(("get-cfg", get_cfg_matches)) => cmd::cmd_spi_get_cfg(get_cfg_matches),
            Some(("write", write_matches)) => cmd::cmd_spi_write(write_matches),
            _ => {
                eprintln!("Unknown SPI command");
                let _ = cli::build_cli().print_help();
                process::exit(1);
            }
        },
        Some(("gpio", sub_matches)) => match sub_matches.subcommand() {
            Some(("get", get_matches)) => cmd::cmd_gpio_get(get_matches),
            Some(("set", set_matches)) => cmd::cmd_gpio_set(set_matches),
            _ => {
                eprintln!("Unknown GPIO command");
                let _ = cli::build_cli().print_help();
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Unknown command");
            let _ = cli::build_cli().print_help();
            process::exit(1);
        }
    }
}
