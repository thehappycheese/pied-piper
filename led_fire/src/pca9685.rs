use std::{thread, time::Duration};
use rppal::i2c::{I2c, Error as I2CError};


/// Bit masks for MODE1 register settings.
#[repr(u8)]
enum M1 {
    RESTART = 0b10000000,
    EXTCLK  = 0b01000000,
    AI      = 0b00100000,
    SLEEP   = 0b00010000,
    SUB1    = 0b00001000,
    SUB2    = 0b00000100,
    SUB3    = 0b00000010,
    ALLCALL = 0b00000001,
    NORMAL  = 0,
}

/// Bit masks for MODE2 register settings.
#[repr(u8)]
enum M2 {
    INVERT  = 0b00010000,
    OCH     = 0b00001000,
    OUTDRV  = 0b00000100,
    OUTNE_H = 0b00000010,
    OUTNE_L = 0b00000001,
}

/// Register addresses
#[repr(u8)]
#[allow(non)]
enum REG {
    MODE1      = 0x00,
    MODE2      = 0x01,
    PRE_SCALE  = 0xFE,
    LED0_ON_L  = 0x06,
    LED0_ON_H  = 0x07,
    LED0_OFF_L = 0x08,
    LED0_OFF_H = 0x09,
}

fn compute_prescale_value_from_hertz(frequency:f32) -> u8 {
    let prescale_val:u8 = ((25_000_000.0 / (4096.0 * frequency)).round() - 1.0) as u8;
    prescale_val
}

pub struct PCA9685{
    device:I2c,
}

impl PCA9685 {

    pub fn new(address:u16, bus:u8) -> Result<Self, I2CError>{
        let mut device = I2c::with_bus(bus)?;
        // Initializes the PCA9685 device with the specified PWM frequency.
       
        device.set_slave_address(address)?;
        device.write(&[REG::MODE1 as u8, M1::SLEEP as u8])?;
        device.write(&[REG::PRE_SCALE as u8, compute_prescale_value_from_hertz(50.0)])?;
        device.write(&[REG::MODE1 as u8, M1::RESTART as u8])?;

        // wait for the device to restart
        thread::sleep(Duration::from_millis(5));
        device.write(&[REG::MODE1 as u8, M1::NORMAL as u8])?; // Otherwise in low power mode by default
        device.write(&[REG::MODE2 as u8, M2::OUTDRV as u8])?; // Output drive configuration

        Ok(PCA9685 {
            device,
        })
    }

    /// Sets the PWM signal for a specific channel.
    /// This function is private.
    fn set_pwm(&mut self, channel: u8, on: u16, off: u16) -> Result<(), I2CError> {
        let base = REG::LED0_ON_L as u8 + 4 * channel;
        self.device.write(&[base, (on & 0xFF) as u8])?;
        self.device.write(&[base + 1, (on >> 8) as u8])?;
        self.device.write(&[base + 2, (off & 0xFF) as u8])?;
        self.device.write(&[base + 3, (off >> 8) as u8])?;
        Ok(())
    }

    pub fn set_servo(&mut self, servo_number:u8, position:f32) -> Result<(), I2CError>{
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