# mp3326-rs

Rust driver for the MPS MP3326 led driver.

## Example
```rust
// I2cDevice is from embassy_embedded_hal
let leds = Mp3326::new(I2cDevice::new(i2c_bus), 0x32);

// Enable Channel 0
let channels_on = Channels::new().with_offset0(true);
leds.set_channel_enables_1_8(channels_on).await?;

// Set channel 0 to half of max led current (channel_current is a 6 bit value)
leds.set_channel_current(0, Current::new().with_channel_current(0b01_1111))
    .await?;

// Set channel 0 pwm to 0. (Pwm is a 12 bit value)
leds.set_channel_pwm(0, Pwm::from(0x000)).await?;

// Set the device config to enable latching when shorts detected, detection threshold at 5V
let control_config = Control::default()
    .with_device_enable(true)
    .with_short_threshold(ShortThreshold::V5);
leds.set_control(control_config).await?;

// Set the led to half brightness
leds.set_channel_pwm(0, Pwm::from(0x7FF)).await?;
```

## Links

- [MP3326 product page](https://www.monolithicpower.com/en/mp3326.html)
- [MP3326 datasheet](https://www.monolithicpower.com/en/documentview/productdocument/index/version/2/document_type/Datasheet/lang/en/sku/MP3326GR/document_id/9567/)

## License

Copyright 2025 Hellbender Inc.

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
