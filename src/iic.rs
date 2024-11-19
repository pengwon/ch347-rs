use libc::{c_int, c_uchar, c_void};

#[link(name = "ch347")]
extern "C" {
    fn CH347I2C_Set(fd: c_int, iMode: c_int) -> c_uchar;
    fn CH347I2C_SetStretch(fd: c_int, enable: c_uchar) -> c_uchar;
    fn CH347I2C_SetDelaymS(fd: c_int, iDelay: c_int) -> c_uchar;
    fn CH347StreamI2C(
        fd: c_int,
        iWriteLength: c_int,
        iWriteBuffer: *const c_void,
        iReadLength: c_int,
        oReadBuffer: *mut c_void,
    ) -> c_uchar;
    fn CH347StreamI2C_RetAck(
        fd: c_int,
        iWriteLength: c_int,
        iWriteBuffer: *const c_void,
        iReadLength: c_int,
        oReadBuffer: *mut c_void,
        retAck: *mut c_int,
    ) -> c_int;
    fn CH347ReadEEPROM(
        fd: c_int,
        iEepromID: c_int,
        iAddr: c_int,
        iLength: c_int,
        oBuffer: *mut u8,
    ) -> c_int;
    fn CH347WriteEEPROM(
        fd: c_int,
        iEepromID: c_int,
        iAddr: c_int,
        iLength: c_int,
        iBuffer: *const u8,
    ) -> c_int;
}

pub fn i2c_set(fd: i32, mode: i32) -> bool {
    unsafe { CH347I2C_Set(fd, mode) != 0 }
}

pub fn i2c_set_stretch(fd: i32, enable: bool) -> bool {
    unsafe { CH347I2C_SetStretch(fd, enable as c_uchar) != 0 }
}

pub fn i2c_set_delay_ms(fd: i32, delay: i32) -> bool {
    unsafe { CH347I2C_SetDelaymS(fd, delay) != 0 }
}

pub fn i2c_stream(fd: i32, write_buffer: &[u8], read_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347StreamI2C(
            fd,
            write_buffer.len() as c_int,
            write_buffer.as_ptr() as *const c_void,
            read_buffer.len() as c_int,
            read_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn i2c_stream_ret_ack(
    fd: i32,
    write_buffer: &[u8],
    read_buffer: &mut [u8],
    ret_ack: &mut i32,
) -> bool {
    unsafe {
        CH347StreamI2C_RetAck(
            fd,
            write_buffer.len() as c_int,
            write_buffer.as_ptr() as *const c_void,
            read_buffer.len() as c_int,
            read_buffer.as_mut_ptr() as *mut c_void,
            ret_ack as *mut c_int,
        ) != 0
    }
}

pub fn read_eeprom(fd: i32, eeprom_id: i32, addr: i32, buffer: &mut [u8]) -> bool {
    unsafe {
        CH347ReadEEPROM(
            fd,
            eeprom_id,
            addr,
            buffer.len() as c_int,
            buffer.as_mut_ptr(),
        ) != 0
    }
}

pub fn write_eeprom(fd: i32, eeprom_id: i32, addr: i32, buffer: &[u8]) -> bool {
    unsafe { CH347WriteEEPROM(fd, eeprom_id, addr, buffer.len() as c_int, buffer.as_ptr()) != 0 }
}
