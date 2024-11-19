# ch347-rs - Rust Command Line Interface for SPI, IIC, and GPIO

## Overview
ch347 is a Rust package that provides a command-line interface to interact with SPI, IIC, and GPIO functionalities. This project encapsulates the necessary operations to initialize, read, write, and close these interfaces, making it easier to manage hardware communication through a simple command-line tool.

## Project Structure
```
ch347-rs
├── src
│   ├── main.rs        # Entry point of the application
│   ├── spi.rs         # SPI functionality implementation
│   ├── iic.rs         # IIC functionality implementation
│   └── gpio.rs        # GPIO functionality implementation
├── Cargo.toml         # Cargo configuration file
└── README.md          # Project documentation
```

## Installation
To build and run the project, ensure you have Rust and Cargo installed on your system. You can download them from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and navigate to the project directory:
```bash
git clone <repository-url>
cd ch347
```

Then, build the project using Cargo:
```bash
cargo build
```

## Usage
After building the project, you can run the command-line application with the following command:
```bash
cargo run -- <command> [options]
```

### Commands
- **SPI**
  - `spi_init`: Initialize the SPI interface.
  - `spi_transfer <data>`: Transfer data over SPI.
  - `spi_close`: Close the SPI interface.

- **IIC**
  - `iic_init`: Initialize the IIC interface.
  - `iic_write <address> <data>`: Write data to the specified IIC address.
  - `iic_read <address>`: Read data from the specified IIC address.
  - `iic_close`: Close the IIC interface.

- **GPIO**
  - `gpio_init`: Initialize GPIO pins.
  - `gpio_set <pin> <value>`: Set the value of a GPIO pin.
  - `gpio_get <pin>`: Get the value of a GPIO pin.
  - `gpio_close`: Close the GPIO interface.

## Examples
1. Initialize SPI:
   ```bash
   cargo run -- spi_init
   ```

2. Transfer data over SPI:
   ```bash
   cargo run -- spi_transfer 0xFF
   ```

3. Write data to an IIC device:
   ```bash
   cargo run -- iic_write 0x50 0xAB
   ```

4. Set a GPIO pin:
   ```bash
   cargo run -- gpio_set 5 1
   ```

## Contributing
Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.