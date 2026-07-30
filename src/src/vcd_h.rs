use super::*;

pub const VCD_SIGNAL_COUNT: i32 = 32;

pub(crate) type TimestampT = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ValueChangeT {
    pub(crate) timestamp: TimestampT,
    pub(crate) value: [i8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SignalT {
    pub(crate) name: [i8; 32],
    pub(crate) size: u64,
    pub(crate) value_changes: [ValueChangeT; 4096],
    pub(crate) changes_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct TimescaleT {
    pub(crate) unit: [i8; 8],
    pub(crate) scale: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VcdT {
    pub(crate) signals_count: u64,
    pub(crate) signals: [SignalT; 32],
    pub(crate) date: [i8; 64],
    pub(crate) version: [i8; 64],
    pub(crate) timescale: TimescaleT,
}
