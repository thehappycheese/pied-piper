use rppal::gpio::Gpio;
use std::{sync::{mpsc::Sender, Arc}, thread, time::{Duration, Instant}};

use crate::config::PiperConfig;


pub enum ButtonToMain {
    Pressed,
}

pub fn poll_button(
    tx: Sender<ButtonToMain>,
    config:Arc<PiperConfig>,
){
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let pin_number = 17;
    let pin = gpio
        .get(pin_number)
        .expect("Failed to get GPIO pin")
        .into_input_pullup();

    let mut last_press_time = Instant::now();

    loop {
        if pin.is_low() {
            thread::sleep(Duration::from_millis(5));
            if pin.is_low() {
                println!("Button Pressed!");
                tx.send(ButtonToMain::Pressed).unwrap();
                last_press_time = Instant::now();
                thread::sleep(Duration::from_millis(1000));
            }
        }else{
            let idle_duration = Instant::now().duration_since(last_press_time);
            let idle_trigger_duration = Duration::from_secs_f32(config.idle_trigger_minutes * 60.0);
            if idle_duration >= idle_trigger_duration {
                println!("Simulating button press due to idle time!");
                tx.send(ButtonToMain::Pressed).unwrap();
                last_press_time = Instant::now();
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}