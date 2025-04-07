#![allow(dead_code)]
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
#![allow(clippy::new_without_default)]

use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier)]
pub enum FPWM {
    Hz220,
    Hz250,
    Hz280,
    Hz330,
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct DimmingFreq {
    #[bits = 2]
    pub fpwm: FPWM,
    #[skip]
    __: B6,
}

#[derive(BitfieldSpecifier)]
pub enum ShortThreshold {
    V2,
    V3,
    V4,
    V5,
}

#[derive(BitfieldSpecifier)]
pub enum Slew {
    None,
    Us5,
    Us10,
    Us20,
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct Control {
    pub device_enable: bool,
    pub phase_shift_enable: bool,
    #[bits = 2]
    pub slew: Slew,
    #[bits = 2]
    pub short_threshold: ShortThreshold,
    pub latch_enable: bool,
    pub fault_enable: bool,
}

impl Default for Control {
    fn default() -> Self {
        Control::from(0x40)
    }
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct OtpAndRefreshFreq {
    pub freq_shift_1_0: B2,
    pub over_temp_fault: bool,
    #[skip]
    __: B5,
}

impl Default for OtpAndRefreshFreq {
    fn default() -> Self {
        OtpAndRefreshFreq::from(0x01)
    }
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct RefreshFreq {
    pub freq_shift_1_0: B8,
}

impl Default for RefreshFreq {
    fn default() -> Self {
        RefreshFreq::from(0x6a)
    }
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct Channels {
    pub offset0: bool,
    pub offset1: bool,
    pub offset2: bool,
    pub offset3: bool,
    pub offset4: bool,
    pub offset5: bool,
    pub offset6: bool,
    pub offset7: bool,
}

impl Default for Channels {
    fn default() -> Self {
        Channels::from(0x00)
    }
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct Current {
    pub channel_current: B6,
    #[skip]
    __: B2,
}

impl Default for Current {
    fn default() -> Self {
        Self::from(0x3f)
    }
}

#[bitfield]
#[repr(u16)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct Pwm {
    pub value: B12,
    #[skip]
    __: B4,
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct PwmMsb {
    pub value: B8,
}

impl From<Pwm> for PwmMsb {
    fn from(value: Pwm) -> Self {
        PwmMsb::from(value.bytes[0])
    }
}

#[bitfield]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, BitfieldSpecifier)]
pub struct PwmLsb {
    pub value: B4,
    #[skip]
    __: B4,
}

impl From<Pwm> for PwmLsb {
    fn from(value: Pwm) -> Self {
        PwmLsb::from(value.bytes[1])
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RegisterError {
    InvalidConversion,
    InvalidOffset,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error<E> {
    Bus(E),
    Register(RegisterError),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::Bus(error)
    }
}
