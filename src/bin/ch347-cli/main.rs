use std::process;

mod cli;
mod cmd;
fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        cmd::list_devices();
    } else if let Some(spi_matches) = matches.subcommand_matches("spi") {
        if let Some(matches) = spi_matches.subcommand_matches("init") {
            cmd::init_spi(matches);
        }
    } else {
        eprintln!("Unknown command");
        let _ = cli::build_cli().print_help();
        process::exit(1);
    }
}
