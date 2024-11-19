use libc::{c_char, c_int, c_uchar, c_ulong, c_ushort, c_void};

#[link(name = "ch347")]
extern "C" {
    fn CH347OpenDevice(dev_index: c_ulong) -> *mut c_void;
    fn CH347CloseDevice(dev_index: c_ulong) -> c_int;
    fn CH347GetDeviceInfor(dev_index: c_ulong, dev_info: *mut DeviceInfo) -> c_int;
    fn CH347GetChipType(dev_index: c_ulong) -> c_uchar;
    fn CH347GetVersion(
        dev_index: c_ulong,
        driver_ver: *mut c_uchar,
        dll_ver: *mut c_uchar,
        bcd_device: *mut c_uchar,
        chip_type: *mut c_uchar,
    ) -> c_int;
    fn CH347SetDeviceNotify(
        dev_index: c_ulong,
        device_id: *const c_char,
        notify_routine: DeviceNotifyRoutine,
    ) -> c_int;
    fn CH347ReadData(dev_index: c_ulong, buffer: *mut c_void, length: *mut c_ulong) -> c_int;
    fn CH347WriteData(dev_index: c_ulong, buffer: *const c_void, length: *mut c_ulong) -> c_int;
    fn CH347SetTimeout(dev_index: c_ulong, write_timeout: c_ulong, read_timeout: c_ulong) -> c_int;
}

#[repr(C)]
pub struct DeviceInfo {
    pub index: c_uchar,
    pub device_path: [c_char; 260],
    pub usb_class: c_uchar,
    pub func_type: c_uchar,
    pub device_id: [c_char; 64],
    pub chip_mode: c_uchar,
    pub dev_handle: *mut c_void,
    pub bulk_out_endp_max_size: c_ushort,
    pub bulk_in_endp_max_size: c_ushort,
    pub usb_speed_type: c_uchar,
    pub ch347_if_num: c_uchar,
    pub data_up_endp: c_uchar,
    pub data_dn_endp: c_uchar,
    pub product_string: [c_char; 64],
    pub manufacturer_string: [c_char; 64],
    pub write_timeout: c_ulong,
    pub read_timeout: c_ulong,
    pub func_desc_str: [c_char; 64],
    pub firmware_ver: c_uchar,
}

pub type DeviceNotifyRoutine = Option<extern "C" fn(event_status: c_ulong)>;

pub fn open_device(dev_index: u32) -> *mut c_void {
    unsafe { CH347OpenDevice(dev_index as c_ulong) }
}

pub fn close_device(dev_index: u32) -> bool {
    unsafe { CH347CloseDevice(dev_index as c_ulong) != 0 }
}

pub fn get_device_info(dev_index: u32, dev_info: &mut DeviceInfo) -> bool {
    unsafe { CH347GetDeviceInfor(dev_index as c_ulong, dev_info as *mut DeviceInfo) != 0 }
}

pub fn get_chip_type(dev_index: u32) -> u8 {
    unsafe { CH347GetChipType(dev_index as c_ulong) }
}

pub fn get_version(
    dev_index: u32,
    driver_ver: &mut u8,
    dll_ver: &mut u8,
    bcd_device: &mut u8,
    chip_type: &mut u8,
) -> bool {
    unsafe {
        CH347GetVersion(
            dev_index as c_ulong,
            driver_ver as *mut u8 as *mut c_uchar,
            dll_ver as *mut u8 as *mut c_uchar,
            bcd_device as *mut u8 as *mut c_uchar,
            chip_type as *mut u8 as *mut c_uchar,
        ) != 0
    }
}

pub fn set_device_notify(
    dev_index: u32,
    device_id: &str,
    notify_routine: DeviceNotifyRoutine,
) -> bool {
    let device_id_cstr = std::ffi::CString::new(device_id).unwrap();
    unsafe {
        CH347SetDeviceNotify(
            dev_index as c_ulong,
            device_id_cstr.as_ptr(),
            notify_routine,
        ) != 0
    }
}

pub fn read_data(dev_index: u32, buffer: &mut [u8]) -> bool {
    let mut length = buffer.len() as c_ulong;
    unsafe {
        CH347ReadData(
            dev_index as c_ulong,
            buffer.as_mut_ptr() as *mut c_void,
            &mut length,
        ) != 0
    }
}

pub fn write_data(dev_index: u32, buffer: &[u8]) -> bool {
    let mut length = buffer.len() as c_ulong;
    unsafe {
        CH347WriteData(
            dev_index as c_ulong,
            buffer.as_ptr() as *const c_void,
            &mut length,
        ) != 0
    }
}

pub fn set_timeout(dev_index: u32, write_timeout: u32, read_timeout: u32) -> bool {
    unsafe {
        CH347SetTimeout(
            dev_index as c_ulong,
            write_timeout as c_ulong,
            read_timeout as c_ulong,
        ) != 0
    }
}