use crate::dev::DeviceInfoRaw;
use libc::{c_int, c_ulong, c_void};

#[link(name = "ch347")]
extern "C" {
    fn CH347Uart_Open(dev_index: c_ulong) -> *mut c_void;
    fn CH347Uart_Close(dev_index: c_ulong) -> c_int;
    fn CH347Uart_GetDeviceInfor(dev_index: c_ulong, dev_info: *mut DeviceInfoRaw) -> c_int;
}

pub fn uart_open(dev_index: u32) -> *mut c_void {
    unsafe { CH347Uart_Open(dev_index) }
}

pub fn uart_close(dev_index: u32) -> bool {
    unsafe { CH347Uart_Close(dev_index) != 0 }
}

pub fn uart_get_device_info(dev_index: u32, dev_info: &mut DeviceInfoRaw) -> bool {
    unsafe { CH347Uart_GetDeviceInfor(dev_index, dev_info as *mut DeviceInfoRaw) != 0 }
}
