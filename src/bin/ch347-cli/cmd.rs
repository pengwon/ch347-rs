use ch347_rs::{
    close_device, get_device_info, gpio_get, gpio_set, open_device, spi_get_cfg, spi_init,
    spi_write, uart_get_device_info, uart_open, DeviceInfo, DeviceInfoRaw, MSpiCfg,
};
use clap::ArgMatches;
use libc::c_void;
use spin_sleep::SpinSleeper;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn list_devices() {
    let mut devices = Vec::new();
    let max_device_number = 16;
    let invalid_handle_value = !0 as *mut c_void;

    for i in 0..max_device_number {
        let mut handle: *mut c_void = uart_open(i);
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

        if handle != invalid_handle_value {
            if uart_get_device_info(i, &mut dev_info_raw) {
                let dev_info = DeviceInfo::from_raw(&dev_info_raw);
                devices.push(dev_info);
            }
        } else {
            handle = open_device(i);
            if handle != invalid_handle_value {
                if get_device_info(i, &mut dev_info_raw) {
                    let dev_info = DeviceInfo::from_raw(&dev_info_raw);
                    devices.push(dev_info);
                }
            } else {
                break;
            }
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

pub fn cmd_gpio_get(matches: &ArgMatches) {
    let fd = *matches.get_one::<u32>("fd").unwrap();
    let mut dir: u8 = 0;
    let mut data: u8 = 0;

    open_device(fd);
    if gpio_get(fd, &mut dir, &mut data) {
        for i in 0..8 {
            let direction = if (dir & (1 << i)) != 0 { "O" } else { "I" };
            let level = if (data & (1 << i)) != 0 { "1" } else { "0" };
            println!("IO{}\t{}\t{}", i, direction, level);
        }
    } else {
        eprintln!("Failed to read GPIO");
    }
}

pub fn cmd_gpio_set(matches: &ArgMatches) {
    let fd = *matches.get_one::<u32>("fd").unwrap();
    let enable = parse_number(matches.get_one::<String>("enable").unwrap()).unwrap();
    let set_dir_out = parse_number(matches.get_one::<String>("set-dir-out").unwrap()).unwrap();
    let set_data_out = parse_number(matches.get_one::<String>("set-data-out").unwrap()).unwrap();

    open_device(fd);
    if gpio_set(fd, enable, set_dir_out, set_data_out) {
        println!("GPIO set successfully");
        let mut dir: u8 = 0;
        let mut data: u8 = 0;
        if gpio_get(fd, &mut dir, &mut data) {
            for i in 0..8 {
                let direction = if (dir & (1 << i)) != 0 { "O" } else { "I" };
                let level = if (data & (1 << i)) != 0 { "1" } else { "0" };
                println!("IO{}\t{}\t{}", i, direction, level);
            }
        } else {
            eprintln!("Failed to read GPIO");
        }
    } else {
        eprintln!("Failed to set GPIO");
    }
}

pub fn cmd_gpio_pwm(matches: &ArgMatches) {
    let fd = *matches.get_one::<u32>("fd").unwrap();
    let channels: Vec<u8> = matches
        .get_many::<String>("channel")
        .unwrap()
        .flat_map(|s| s.split(','))
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let frequency = *matches.get_one::<u32>("frequency").unwrap();
    let duty_cycles: Vec<u8> = matches
        .get_many::<String>("duty_cycle")
        .unwrap()
        .flat_map(|s| s.split(','))
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let pulse_count = *matches.get_one::<u32>("pulse_count").unwrap();

    // 调用设置 PWM 的函数
    open_device(fd);
    if !set_pwm(fd, &channels, frequency, &duty_cycles, pulse_count) {
        eprintln!("Failed to set PWM");
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

fn set_pwm(fd: u32, channels: &[u8], frequency: u32, duty_cycles: &[u8], pulse_count: u32) -> bool {
    let enable: u8 = channels.iter().fold(0, |acc, &ch| acc | (1 << ch)); // 启用 GPIO
    let dir_out: u8 = enable; // 设置为输出
    let period = 1_000_000 / frequency; // 周期时间（微秒）

    // 在 PWM 输出前将 GPIO 初始化为低电平
    if !gpio_set(fd, enable, dir_out, 0) {
        return false;
    }

    // 构造一个 SpinSleeper：忙等阈值 100µs，超过则调用 thread::sleep
    let sleeper =
        SpinSleeper::new(100000).with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);

    // 将通道按照占空比进行分组
    let mut duty_cycle_to_channels: HashMap<u8, Vec<u8>> = HashMap::new();
    for (i, &channel) in channels.iter().enumerate() {
        let duty_cycle_index = if i < duty_cycles.len() {
            i
        } else {
            duty_cycles.len() - 1
        };
        let duty_cycle = duty_cycles[duty_cycle_index];
        duty_cycle_to_channels
            .entry(duty_cycle)
            .or_insert(Vec::new())
            .push(channel);
    }

    // 计算每个占空比对应的时段结束时间
    let mut times: Vec<(u32, u8)> = Vec::new();
    let mut cumulative_time = 0;
    let mut duty_cycle_values: Vec<&u8> = duty_cycle_to_channels.keys().collect();
    duty_cycle_values.sort();

    let mut data_out = 0; // 初始状态，所有通道低电平
    times.push((0, data_out)); // 起始事件

    for &dc in &duty_cycle_values {
        let duration = period * (*dc as u32) / 100;
        cumulative_time += duration;

        // 在对应时段开始时将对应通道置高
        let channels = &duty_cycle_to_channels[dc];
        let channels_mask = channels.iter().fold(0, |acc, &ch| acc | (1 << ch));
        data_out |= channels_mask;
        times.push((cumulative_time - duration, data_out));

        // 在对应时段结束时将对应通道置低
        data_out &= !channels_mask;
        times.push((cumulative_time, data_out));
    }

    // 确保最后一个时段结束在周期结束处
    if cumulative_time < period {
        times.push((period, 0));
    }

    // 按时间排序事件
    times.sort_by_key(|&(time, _)| time);

    // 开始 PWM 信号输出
    // 用闭包输出一次完整周期
    let output_pwm = |times: &[(u32, u8)]| -> bool {
        let start = Instant::now();
        for &(time, state) in times {
            let target = start + Duration::from_micros(time as u64);
            sleeper.sleep(target - Instant::now());
            if !gpio_set(fd, enable, dir_out, state) {
                return false;
            }
        }
        true
    };

    if pulse_count == 0 {
        // 连续输出
        loop {
            if !output_pwm(&times) {
                return false;
            }
        }
    } else {
        // 输出指定脉冲数
        for _ in 0..pulse_count {
            if !output_pwm(&times) {
                return false;
            }
        }
    }

    true
}
