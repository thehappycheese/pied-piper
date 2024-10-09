use std::{thread, time::Duration};
use rppal::i2c::{I2c, Error as I2CError};

/// Bit masks for MODE1 register settings.
const MODE1_RESTART: u8 = 0b10000000;
// const MODE1_EXTCLK : u8 = 0b01000000;
// const MODE1_AI     : u8 = 0b00100000;
const MODE1_SLEEP  : u8 = 0b00010000;
// const MODE1_SUB1   : u8 = 0b00001000;
// const MODE1_SUB2   : u8 = 0b00000100;
// const MODE1_SUB3   : u8 = 0b00000010;
// const MODE1_ALLCALL: u8 = 0b00000001;
const MODE1_NORMAL : u8 = 0b00000000;

/// Bit masks for MODE2 register settings.
// const MODE2_INVERT : u8 = 0b00010000;
// const MODE2_OCH    : u8 = 0b00001000;
const MODE2_OUTDRV : u8 = 0b00000100;
// const MODE2_OUTNE_H: u8 = 0b00000010;
// const MODE2_OUTNE_L: u8 = 0b00000001;

/// Register addresses
const REG_MODE1     : u8 = 0x00;
const REG_MODE2     : u8 = 0x01;
const REG_PRE_SCALE : u8 = 0xFE;
const REG_LED0_ON_L : u8 = 0x06;
// const REG_LED0_ON_H : u8 = 0x07;
// const REG_LED0_OFF_L: u8 = 0x08;
// const REG_LED0_OFF_H: u8 = 0x09;

fn compute_prescale_value_from_hertz(frequency: f32) -> u8 {
    let prescale_val: u8 = ((25_000_000.0 / (4096.0 * frequency)).round() - 1.0) as u8;
    prescale_val
}

pub struct PCA9685 {
    device: I2c,
}

impl PCA9685 {
    pub fn new(address: u16, bus: u8) -> Result<Self, I2CError> {
        let mut device = I2c::with_bus(bus)?;
        // Initializes the PCA9685 device with the specified PWM frequency.
       
        device.set_slave_address(address)?;
        device.write(&[REG_MODE1, MODE1_SLEEP])?;
        device.write(&[REG_PRE_SCALE, compute_prescale_value_from_hertz(50.0)])?;
        device.write(&[REG_MODE1, MODE1_RESTART])?;

        // wait for the device to restart
        thread::sleep(Duration::from_millis(5));
        device.write(&[REG_MODE1, MODE1_NORMAL])?; // Otherwise in low power mode by default
        device.write(&[REG_MODE2, MODE2_OUTDRV])?; // Output drive configuration

        Ok(PCA9685 { device })
    }

    /// Sets the PWM signal for a specific channel.
    /// This function is private.
    fn set_pwm(&mut self, channel: u8, on: u16, off: u16) -> Result<(), I2CError> {
        let base = REG_LED0_ON_L + 4 * channel;
        self.device.write(&[base, (on & 0xFF) as u8])?;
        self.device.write(&[base + 1, (on >> 8) as u8])?;
        self.device.write(&[base + 2, (off & 0xFF) as u8])?;
        self.device.write(&[base + 3, (off >> 8) as u8])?;
        Ok(())
    }

    pub fn set_servo(&mut self, servo_number: u8, position: f32) -> Result<(), I2CError> {
        assert!(servo_number < 16, "Servo number must be between 0 and 15");
        assert!(
            position >= 0.0 && position <= 1.0,
            "Position must be between 0.0 and 1.0"
        );
        
        let pulse_length = 0.05 + 0.05 * position;
        let pulse = (pulse_length * 4096.0).round() as u16;
        self.set_pwm(servo_number, 0, pulse)
    }
}
