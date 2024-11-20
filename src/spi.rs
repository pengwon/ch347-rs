use libc::{c_uchar, c_uint, c_ulong, c_ushort, c_void};

#[repr(C)]
pub struct MSpiCfg {
    pub mode: u8,
    pub clock: u8,
    pub byte_order: u8,
    pub spi_write_read_interval: u16,
    pub spi_out_default_data: u8,
    pub chip_select: u32,
    pub cs1_polarity: u8,
    pub cs2_polarity: u8,
    pub is_auto_deactive_cs: u16,
    pub active_delay: u16,
    pub delay_deactive: u32,
}

#[link(name = "ch347")]
extern "C" {
    fn CH347SPI_SetChipSelect(
        fd: c_ulong,
        iEnableSelect: c_ushort,
        iChipSelect: c_ushort,
        iIsAutoDeativeCS: c_uint,
        iActiveDelay: c_uint,
        iDelayDeactive: c_uint,
    ) -> c_ulong;

    fn CH347SPI_Write(
        fd: c_ulong,
        iChipSelect: c_uint,
        iLength: c_uint,
        iWriteStep: c_uint,
        ioBuffer: *mut c_void,
    ) -> c_ulong;

    fn CH347SPI_Read(
        fd: c_ulong,
        iChipSelect: c_uint,
        oLength: c_uint,
        iLength: *mut c_uint,
        ioBuffer: *mut c_void,
    ) -> c_ulong;

    fn CH347SPI_WriteRead(
        fd: c_ulong,
        iChipSelect: c_uint,
        iLength: c_uint,
        ioBuffer: *mut c_void,
    ) -> c_ulong;

    fn CH347SPI_SetFrequency(fd: c_ulong, iSpiSpeedHz: c_uint) -> c_ulong;

    fn CH347SPI_SetDataBits(fd: c_ulong, iDataBits: c_uchar) -> c_ulong;

    fn CH347SPI_Init(fd: c_ulong, SpiCfg: *mut MSpiCfg) -> c_ulong;

    fn CH347SPI_GetCfg(fd: c_ulong, SpiCfg: *mut MSpiCfg) -> c_ulong;
}

pub fn spi_set_chip_select(
    fd: u32,
    enable_select: u16,
    chip_select: u16,
    is_auto_deactive_cs: u32,
    active_delay: u32,
    delay_deactive: u32,
) -> bool {
    unsafe {
        CH347SPI_SetChipSelect(
            fd,
            enable_select,
            chip_select,
            is_auto_deactive_cs,
            active_delay,
            delay_deactive,
        ) != 0
    }
}

pub fn spi_write(
    fd: u32,
    chip_select: u32,
    length: u32,
    write_step: u32,
    buffer: &mut [u8],
) -> bool {
    unsafe {
        CH347SPI_Write(
            fd,
            chip_select,
            length,
            write_step,
            buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn spi_read(fd: u32, chip_select: u32, length: u32, buffer: &mut [u8]) -> bool {
    let mut read_length: u32 = 0;
    unsafe {
        CH347SPI_Read(
            fd,
            chip_select,
            length,
            &mut read_length,
            buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn spi_write_read(fd: u32, chip_select: u32, length: u32, buffer: &mut [u8]) -> bool {
    unsafe { CH347SPI_WriteRead(fd, chip_select, length, buffer.as_mut_ptr() as *mut c_void) != 0 }
}

pub fn spi_set_frequency(fd: u32, spi_speed_hz: u32) -> bool {
    unsafe { CH347SPI_SetFrequency(fd, spi_speed_hz) != 0 }
}

pub fn spi_set_data_bits(fd: u32, data_bits: u8) -> bool {
    unsafe { CH347SPI_SetDataBits(fd, data_bits) != 0 }
}

pub fn spi_init(fd: u32, spi_cfg: &mut MSpiCfg) -> bool {
    unsafe { CH347SPI_Init(fd, spi_cfg as *mut MSpiCfg) != 0 }
}

pub fn spi_get_cfg(fd: u32, spi_cfg: &mut MSpiCfg) -> bool {
    unsafe { CH347SPI_GetCfg(fd, spi_cfg as *mut MSpiCfg) != 0 }
}
