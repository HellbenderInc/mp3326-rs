#![allow(dead_code)]

use crate::types::*;

const NUM_OFFSETS: u8 = 16;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, defmt::Format)]
pub enum Register {
    DimmingFreq,
    Control,
    OtpAndRefreshFreq,
    RefreshFreq,
    ChannelEnable0,
    ChannelEnable1,
    ChannelOpenFault0,
    ChannelOpenFault1,
    ChannelShortFault0,
    ChannelShortFault1,
    ChannelCurrentStart,
    ChannelPwmDutyLsbStart,
    ChannelPwmDutyMsbStart,
}

impl Register {
    pub fn addr(&self) -> Result<u8, RegisterError> {
        match self {
            Register::DimmingFreq => Ok(0x00),
            Register::Control => Ok(0x01),
            Register::OtpAndRefreshFreq => Ok(0x02),
            Register::RefreshFreq => Ok(0x03),
            Register::ChannelEnable0 => Ok(0x04),
            Register::ChannelEnable1 => Ok(0x05),
            Register::ChannelOpenFault0 => Ok(0x06),
            Register::ChannelOpenFault1 => Ok(0x07),
            Register::ChannelShortFault0 => Ok(0x08),
            Register::ChannelShortFault1 => Ok(0x09),
            _ => Err(RegisterError::InvalidConversion),
        }
    }

    pub fn offset_addr(&self, offset: u8) -> Result<u8, RegisterError> {
        let used_offset = if offset >= NUM_OFFSETS {
            return Err(RegisterError::InvalidOffset);
        } else {
            offset
        };

        match self {
            Register::ChannelCurrentStart => Ok(0x0a + (3 * used_offset)),
            Register::ChannelPwmDutyLsbStart => Ok(0x0b + (3 * used_offset)),
            Register::ChannelPwmDutyMsbStart => Ok(0x0c + (3 * used_offset)),
            _ => Err(RegisterError::InvalidConversion),
        }
    }
}
