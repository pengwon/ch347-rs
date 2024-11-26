use libc::{c_int, c_uchar, c_ulong};

#[link(name = "ch347")]
extern "C" {
    fn CH347GPIO_Get(fd: c_ulong, iDir: *mut c_uchar, iData: *mut c_uchar) -> c_int;
    fn CH347GPIO_Set(
        fd: c_ulong,
        iEnable: c_uchar,
        iSetDirOut: c_uchar,
        iSetDataOut: c_uchar,
    ) -> c_int;
}

pub fn gpio_get(fd: u32, dir: &mut u8, data: &mut u8) -> bool {
    unsafe {
        CH347GPIO_Get(
            fd,
            dir as *mut u8 as *mut c_uchar,
            data as *mut u8 as *mut c_uchar,
        ) != 0
    }
}

pub fn gpio_set(fd: u32, enable: u8, set_dir_out: u8, set_data_out: u8) -> bool {
    unsafe {
        CH347GPIO_Set(
            fd,
            enable as c_uchar,
            set_dir_out as c_uchar,
            set_data_out as c_uchar,
        ) != 0
    }
}
