use ch347_rs::{
    close_device, get_device_info, open_device, spi_init, DeviceInfo, DeviceInfoRaw, MSpiCfg,
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

        close_device(i);
    }

    for device in devices {
        println!("{}", device);
    }
}

pub fn init_spi(matches: &ArgMatches) {
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

    open_device(fd);
    if spi_init(fd, &mut spi_cfg) {
        println!("SPI initialized successfully");
    } else {
        eprintln!("Failed to initialize SPI");
    }
}
