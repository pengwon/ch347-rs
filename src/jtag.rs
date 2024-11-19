use libc::{c_int, c_uchar, c_ulong, c_void};

#[link(name = "ch347")]
extern "C" {
    fn CH347Jtag_INIT(iIndex: c_ulong, iClockRate: c_uchar) -> c_int;
    fn CH347Jtag_GetCfg(iIndex: c_ulong, ClockRate: *mut c_uchar) -> c_int;
    fn CH347Jtag_TmsChange(
        iIndex: c_ulong,
        tmsValue: *mut c_uchar,
        Step: c_ulong,
        Skip: c_ulong,
    ) -> c_int;
    fn CH347Jtag_IoScan(
        iIndex: c_ulong,
        DataBits: *mut c_uchar,
        DataBitsNb: c_ulong,
        IsRead: c_int,
    ) -> c_int;
    fn CH347Jtag_IoScanT(
        iIndex: c_ulong,
        DataBits: *mut c_uchar,
        DataBitsNb: c_ulong,
        IsRead: c_int,
        IsLastPkt: c_int,
    ) -> c_int;
    fn CH347Jtag_Reset(iIndex: c_ulong) -> c_ulong;
    fn CH347Jtag_ResetTrst(iIndex: c_ulong, TRSTLevel: c_int) -> c_int;
    fn CH347Jtag_WriteRead(
        iIndex: c_ulong,
        IsDR: c_int,
        iWriteBitLength: c_ulong,
        iWriteBitBuffer: *mut c_void,
        oReadBitLength: *mut c_ulong,
        oReadBitBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_WriteRead_Fast(
        iIndex: c_ulong,
        IsDR: c_int,
        iWriteBitLength: c_ulong,
        iWriteBitBuffer: *mut c_void,
        oReadBitLength: *mut c_ulong,
        oReadBitBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_WriteReadEx(
        iIndex: c_ulong,
        IsInDrOrIr: c_int,
        IsDR: c_int,
        iWriteBitLength: c_ulong,
        iWriteBitBuffer: *mut c_void,
        oReadBitLength: *mut c_ulong,
        oReadBitBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_WriteRead_FastEx(
        iIndex: c_ulong,
        IsInDrOrIr: c_int,
        IsDR: c_int,
        iWriteBitLength: c_ulong,
        iWriteBitBuffer: *mut c_void,
        oReadBitLength: *mut c_ulong,
        oReadBitBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_SwitchTapState(TapState: c_uchar) -> c_int;
    fn CH347Jtag_ByteWriteDR(
        iIndex: c_ulong,
        iWriteLength: c_ulong,
        iWriteBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_ByteReadDR(
        iIndex: c_ulong,
        oReadLength: *mut c_ulong,
        oReadBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_ByteWriteIR(
        iIndex: c_ulong,
        iWriteLength: c_ulong,
        iWriteBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_ByteReadIR(
        iIndex: c_ulong,
        oReadLength: *mut c_ulong,
        oReadBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_BitWriteDR(
        iIndex: c_ulong,
        iWriteBitLength: c_ulong,
        iWriteBitBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_BitWriteIR(
        iIndex: c_ulong,
        iWriteBitLength: c_ulong,
        iWriteBitBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_BitReadIR(
        iIndex: c_ulong,
        oReadBitLength: *mut c_ulong,
        oReadBitBuffer: *mut c_void,
    ) -> c_int;
    fn CH347Jtag_BitReadDR(
        iIndex: c_ulong,
        oReadBitLength: *mut c_ulong,
        oReadBitBuffer: *mut c_void,
    ) -> c_int;
}

pub fn jtag_init(index: u32, clock_rate: u8) -> bool {
    unsafe { CH347Jtag_INIT(index as c_ulong, clock_rate as c_uchar) != 0 }
}

pub fn jtag_get_cfg(index: u32, clock_rate: &mut u8) -> bool {
    unsafe { CH347Jtag_GetCfg(index as c_ulong, clock_rate as *mut u8 as *mut c_uchar) != 0 }
}

pub fn jtag_tms_change(index: u32, tms_value: &mut [u8], step: u32, skip: u32) -> bool {
    unsafe {
        CH347Jtag_TmsChange(
            index as c_ulong,
            tms_value.as_mut_ptr(),
            step as c_ulong,
            skip as c_ulong,
        ) != 0
    }
}

pub fn jtag_io_scan(index: u32, data_bits: &mut [u8], data_bits_nb: u32, is_read: bool) -> bool {
    unsafe {
        CH347Jtag_IoScan(
            index as c_ulong,
            data_bits.as_mut_ptr(),
            data_bits_nb as c_ulong,
            is_read as c_int,
        ) != 0
    }
}

pub fn jtag_io_scan_t(
    index: u32,
    data_bits: &mut [u8],
    data_bits_nb: u32,
    is_read: bool,
    is_last_pkt: bool,
) -> bool {
    unsafe {
        CH347Jtag_IoScanT(
            index as c_ulong,
            data_bits.as_mut_ptr(),
            data_bits_nb as c_ulong,
            is_read as c_int,
            is_last_pkt as c_int,
        ) != 0
    }
}

pub fn jtag_reset(index: u32) -> u32 {
    unsafe { CH347Jtag_Reset(index as c_ulong) as u32 }
}

pub fn jtag_reset_trst(index: u32, trst_level: bool) -> bool {
    unsafe { CH347Jtag_ResetTrst(index as c_ulong, trst_level as c_int) != 0 }
}

pub fn jtag_write_read(
    index: u32,
    is_dr: bool,
    write_bit_length: u32,
    write_bit_buffer: &mut [u8],
    read_bit_length: &mut u32,
    read_bit_buffer: &mut [u8],
) -> bool {
    unsafe {
        CH347Jtag_WriteRead(
            index as c_ulong,
            is_dr as c_int,
            write_bit_length as c_ulong,
            write_bit_buffer.as_mut_ptr() as *mut c_void,
            read_bit_length as *mut u32 as *mut c_ulong,
            read_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_write_read_fast(
    index: u32,
    is_dr: bool,
    write_bit_length: u32,
    write_bit_buffer: &mut [u8],
    read_bit_length: &mut u32,
    read_bit_buffer: &mut [u8],
) -> bool {
    unsafe {
        CH347Jtag_WriteRead_Fast(
            index as c_ulong,
            is_dr as c_int,
            write_bit_length as c_ulong,
            write_bit_buffer.as_mut_ptr() as *mut c_void,
            read_bit_length as *mut u32 as *mut c_ulong,
            read_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_write_read_ex(
    index: u32,
    is_in_dr_or_ir: bool,
    is_dr: bool,
    write_bit_length: u32,
    write_bit_buffer: &mut [u8],
    read_bit_length: &mut u32,
    read_bit_buffer: &mut [u8],
) -> bool {
    unsafe {
        CH347Jtag_WriteReadEx(
            index as c_ulong,
            is_in_dr_or_ir as c_int,
            is_dr as c_int,
            write_bit_length as c_ulong,
            write_bit_buffer.as_mut_ptr() as *mut c_void,
            read_bit_length as *mut u32 as *mut c_ulong,
            read_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_write_read_fast_ex(
    index: u32,
    is_in_dr_or_ir: bool,
    is_dr: bool,
    write_bit_length: u32,
    write_bit_buffer: &mut [u8],
    read_bit_length: &mut u32,
    read_bit_buffer: &mut [u8],
) -> bool {
    unsafe {
        CH347Jtag_WriteRead_FastEx(
            index as c_ulong,
            is_in_dr_or_ir as c_int,
            is_dr as c_int,
            write_bit_length as c_ulong,
            write_bit_buffer.as_mut_ptr() as *mut c_void,
            read_bit_length as *mut u32 as *mut c_ulong,
            read_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_switch_tap_state(tap_state: u8) -> bool {
    unsafe { CH347Jtag_SwitchTapState(tap_state as c_uchar) != 0 }
}

pub fn jtag_byte_write_dr(index: u32, write_length: u32, write_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_ByteWriteDR(
            index as c_ulong,
            write_length as c_ulong,
            write_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_byte_read_dr(index: u32, read_length: &mut u32, read_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_ByteReadDR(
            index as c_ulong,
            read_length as *mut u32 as *mut c_ulong,
            read_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_byte_write_ir(index: u32, write_length: u32, write_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_ByteWriteIR(
            index as c_ulong,
            write_length as c_ulong,
            write_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_byte_read_ir(index: u32, read_length: &mut u32, read_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_ByteReadIR(
            index as c_ulong,
            read_length as *mut u32 as *mut c_ulong,
            read_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_bit_write_dr(index: u32, write_bit_length: u32, write_bit_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_BitWriteDR(
            index as c_ulong,
            write_bit_length as c_ulong,
            write_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_bit_write_ir(index: u32, write_bit_length: u32, write_bit_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_BitWriteIR(
            index as c_ulong,
            write_bit_length as c_ulong,
            write_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_bit_read_ir(index: u32, read_bit_length: &mut u32, read_bit_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_BitReadIR(
            index as c_ulong,
            read_bit_length as *mut u32 as *mut c_ulong,
            read_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}

pub fn jtag_bit_read_dr(index: u32, read_bit_length: &mut u32, read_bit_buffer: &mut [u8]) -> bool {
    unsafe {
        CH347Jtag_BitReadDR(
            index as c_ulong,
            read_bit_length as *mut u32 as *mut c_ulong,
            read_bit_buffer.as_mut_ptr() as *mut c_void,
        ) != 0
    }
}
