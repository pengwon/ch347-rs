use ch347_rs::{
    close_device, get_device_info, open_device, spi_get_cfg, spi_init, spi_write, DeviceInfo,
    DeviceInfoRaw, MSpiCfg,
};
use clap::ArgMatches;
use libc::c_void;

pub fn list_devices() {
    let mut devices = Vec::new();
    let max_device_number = 16;
    let invalid_handle_value = !0 as *mut c_void;

    for i in 0..max_device_number {
        let handle = open_device(i);
        if handle == invalid_handle_value {
            break;
        };

        let mut dev_info_raw = DeviceInfoRaw {
            index: 0,
            device_path: [0; 260],
            usb_class: 0,
            func_type: 0,
            device_id: [0; 64],
            chip_mode: 0,
            dev_handle: std::ptr::null_mut(),
            bulk_out_endp_max_size: 0,
            bulk_in_endp_max_size: 0,
            usb_speed_type: 0,
            ch347_if_num: 0,
            data_up_endp: 0,
            data_dn_endp: 0,
            product_string: [0; 64],
            manufacturer_string: [0; 64],
            write_timeout: 0,
            read_timeout: 0,
            func_desc_str: [0; 64],
            firmware_ver: 0,
        };

        if get_device_info(i, &mut dev_info_raw) != false {
            let dev_info = DeviceInfo::from_raw(&dev_info_raw);
            devices.push(dev_info);
        }
    }

    for device in devices {
        println!("{}", device);
    }
}

pub fn close_devices(matches: &ArgMatches) {
    let fd = *matches.get_one::<u32>("fd").unwrap();
    let close_all = matches.get_flag("all");
    let max_device_number = 16;

    if close_all {
        for i in 0..max_device_number {
            close_device(i);
        }
        println!("All devices closed.");
    } else {
        close_device(fd);
        println!("Device {} closed.", fd);
    }
}

pub fn cmd_spi_init(matches: &ArgMatches) {
    let fd = *matches.get_one::<u32>("fd").unwrap();
    let mode = *matches.get_one::<u8>("mode").unwrap();
    let clock = *matches.get_one::<u8>("clock").unwrap();
    let byte_order = *matches.get_one::<u8>("byte_order").unwrap();
    let spi_write_read_interval = *matches.get_one::<u16>("spi_write_read_interval").unwrap();
    let spi_out_default_data = *matches.get_one::<u8>("spi_out_default_data").unwrap();
    let chip_select = *matches.get_one::<u32>("chip_select").unwrap();
    let cs1_polarity = *matches.get_one::<u8>("cs1_polarity").unwrap();
    let cs2_polarity = *matches.get_one::<u8>("cs2_polarity").unwrap();
    let is_auto_deactive_cs = *matches.get_one::<u16>("is_auto_deactive_cs").unwrap();
    let active_delay = *matches.get_one::<u16>("active_delay").unwrap();
    let delay_deactive = *matches.get_one::<u32>("delay_deactive").unwrap();

    let mut spi_cfg = MSpiCfg {
        mode,
        clock,
        byte_order,
        spi_write_read_interval,
        spi_out_default_data,
        chip_select,
        cs1_polarity,
        cs2_polarity,
        is_auto_deactive_cs,
        active_delay,
        delay_deactive,
    };
    println!("{}", spi_cfg.byte_order);
    open_device(fd);
    if spi_init(fd, &mut spi_cfg) {
        println!("SPI initialized successfully");
    } else {
        eprintln!("Failed to initialize SPI");
    }
}

pub fn cmd_spi_get_cfg(matches: &ArgMatches) {
    let fd = *matches.get_one::<u32>("fd").unwrap();
    let mut spi_cfg = MSpiCfg {
        mode: 0,
        clock: 0,
        byte_order: 0,
        spi_write_read_interval: 0,
        spi_out_default_data: 0,
        chip_select: 0,
        cs1_polarity: 0,
        cs2_polarity: 0,
        is_auto_deactive_cs: 0,
        active_delay: 0,
        delay_deactive: 0,
    };
    open_device(fd);
    if spi_get_cfg(fd, &mut spi_cfg) {
        println!("SPI configuration:");
        println!("Mode: {}", spi_cfg.mode);
        println!("Clock: {}", spi_cfg.clock);
        println!("Byte order: {}", spi_cfg.byte_order);
        println!(
            "SPI write-read interval: {}",
            spi_cfg.spi_write_read_interval
        );
        println!("SPI out default data: {}", spi_cfg.spi_out_default_data);
        println!("Chip select: {}", spi_cfg.chip_select);
        println!("CS1 polarity: {}", spi_cfg.cs1_polarity);
        println!("CS2 polarity: {}", spi_cfg.cs2_polarity);
        println!("Is auto deactive CS: {}", spi_cfg.is_auto_deactive_cs);
        println!("Active delay: {}", spi_cfg.active_delay);
        println!("Delay deactive: {}", spi_cfg.delay_deactive);
    } else {
        eprintln!("Failed to get SPI configuration");
    }
}

pub fn cmd_spi_write(matches: &ArgMatches) {
    let fd = *matches.get_one::<u32>("fd").unwrap();
    let chip_select_input = *matches.get_one::<u32>("chip_select").unwrap();
    let chip_select = match chip_select_input {
        0 => 0,
        1 => 0x80,
        2 => 0x81,
        _ => {
            eprintln!("Invalid chip select value: {}", chip_select_input);
            return;
        }
    };
    let write_step = *matches.get_one::<u32>("write_step").unwrap();
    let data = matches.get_one::<String>("data").unwrap();

    let mut buffer = match parse_data(data) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to parse data: {}", e);
            return;
        }
    };
    let length = buffer.len() as u32;
    open_device(fd);
    if spi_write(fd, chip_select, length, write_step, &mut buffer) {
        println!("SPI write successful");
    } else {
        eprintln!("SPI write failed");
    }
}

fn parse_number(s: &str) -> Result<u8, String> {
    if s.starts_with("0x") {
        u8::from_str_radix(&s[2..], 16).map_err(|e| e.to_string())
    } else if s.starts_with("0b") {
        u8::from_str_radix(&s[2..], 2).map_err(|e| e.to_string())
    } else if s.starts_with("0o") {
        u8::from_str_radix(&s[2..], 8).map_err(|e| e.to_string())
    } else {
        s.parse::<u8>().map_err(|e| e.to_string())
    }
}

fn parse_data(data: &str) -> Result<Vec<u8>, String> {
    data.split(|c| c == ' ' || c == ',')
        .map(|s| parse_number(s.trim()))
        .collect()
}
