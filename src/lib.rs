#![no_std]
#![allow(dead_code)]
#![doc = include_str!("../README.md")]

use embedded_hal::i2c::ErrorType;
use embedded_hal_async::i2c::{I2c, SevenBitAddress};

mod regs;
pub mod types;

use regs::*;
use types::{PwmLsb, PwmMsb};

macro_rules! gen_pub_register_setter_fn {
    ($name:tt, $input_type:ty, $reg:expr) => {
        pub async fn $name(&mut self, input: $input_type) -> Result<(), types::Error<E>> {
            let write_buffer: [u8; 2] = [$reg.addr().unwrap(), input.into()];

            self.write(&write_buffer).await?;

            Ok(())
        }
    };
}

macro_rules! gen_pub_register_getter_fn {
    ($name:tt, $ret_type:ty, $reg:expr) => {
        pub async fn $name(&mut self) -> Result<$ret_type, types::Error<E>> {
            let write_buffer: [u8; 1] = [$reg.addr().unwrap()];
            let mut read_buffer: [u8; 1] = [0];

            self.write_read(&write_buffer, &mut read_buffer).await?;

            Ok(<$ret_type>::from(read_buffer[0]))
        }
    };
}

macro_rules! gen_pub_register_channel_setter_fn {
    ($name:tt, $input_type:ty, $reg:expr) => {
        pub async fn $name(
            &mut self,
            offset: u8,
            value: $input_type,
        ) -> Result<(), types::Error<E>> {
            let write_buffer: [u8; 2] = [$reg.offset_addr(offset).unwrap(), value.into()];

            self.write(&write_buffer).await?;

            Ok(())
        }
    };
}

pub struct Mp3326<BUS: I2c>
where
    BUS: I2c,
{
    bus: BUS,
    addr: u8,
}

impl<BUS, E> Mp3326<BUS>
where
    BUS: I2c<SevenBitAddress, Error = E>,
{
    pub fn new(bus: BUS, addr: u8) -> Self {
        Self { bus, addr }
    }

    // Setters
    gen_pub_register_setter_fn!(set_dimming_freq, types::DimmingFreq, Register::DimmingFreq);
    gen_pub_register_setter_fn!(set_control, types::Control, Register::Control);
    gen_pub_register_setter_fn!(
        set_opt_and_refresh_freq,
        types::OtpAndRefreshFreq,
        Register::OtpAndRefreshFreq
    );
    gen_pub_register_setter_fn!(set_refresh_freq, types::RefreshFreq, Register::RefreshFreq);
    gen_pub_register_setter_fn!(
        set_channel_enables_9_16,
        types::Channels,
        Register::ChannelEnable0
    );
    gen_pub_register_setter_fn!(
        set_channel_enables_1_8,
        types::Channels,
        Register::ChannelEnable1
    );
    gen_pub_register_channel_setter_fn!(
        set_channel_current,
        types::Current,
        Register::ChannelCurrentStart
    );
    gen_pub_register_channel_setter_fn!(
        set_channel_pwm_msb,
        types::PwmMsb,
        Register::ChannelPwmDutyMsbStart
    );
    gen_pub_register_channel_setter_fn!(
        set_channel_pwm_lsb,
        types::PwmLsb,
        Register::ChannelPwmDutyLsbStart
    );

    // Getters
    gen_pub_register_getter_fn!(get_dimming_freq, types::DimmingFreq, Register::DimmingFreq);
    gen_pub_register_getter_fn!(get_control, types::Control, Register::Control);
    gen_pub_register_getter_fn!(
        get_opt_and_refresh_freq,
        types::OtpAndRefreshFreq,
        Register::OtpAndRefreshFreq
    );
    gen_pub_register_getter_fn!(get_refresh_freq, types::RefreshFreq, Register::RefreshFreq);
    gen_pub_register_getter_fn!(
        get_channel_enables_9_16,
        types::Channels,
        Register::ChannelEnable0
    );
    gen_pub_register_getter_fn!(
        get_channel_enables_1_8,
        types::Channels,
        Register::ChannelEnable0
    );
    gen_pub_register_getter_fn!(
        get_channel_open_fault_9_16,
        types::Channels,
        Register::ChannelOpenFault0
    );
    gen_pub_register_getter_fn!(
        get_channel_open_fault_1_8,
        types::Channels,
        Register::ChannelOpenFault1
    );
    gen_pub_register_getter_fn!(
        get_channel_short_fault_9_16,
        types::Channels,
        Register::ChannelShortFault0
    );
    gen_pub_register_getter_fn!(
        get_channel_short_fault_1_8,
        types::Channels,
        Register::ChannelShortFault1
    );

    pub async fn set_channel_pwm(
        &mut self,
        offset: u8,
        value: types::Pwm,
    ) -> Result<(), types::Error<E>> {
        let msb: PwmMsb = PwmMsb::from(value);
        let lsb: PwmLsb = PwmLsb::from(value);

        // output only updates when msb is written, so always write msb last
        self.set_channel_pwm_lsb(offset, lsb).await?;
        self.set_channel_pwm_msb(offset, msb).await?;

        Ok(())
    }

    #[inline]
    async fn write(&mut self, buffer: &[u8]) -> Result<(), <BUS as ErrorType>::Error> {
        self.bus.write(self.addr, buffer).await
    }

    #[inline]
    async fn write_read(
        &mut self,
        write_buffer: &[u8],
        read_buffer: &mut [u8],
    ) -> Result<(), <BUS as ErrorType>::Error> {
        self.bus
            .write_read(self.addr, write_buffer, read_buffer)
            .await
    }
}
