use rppal::gpio::Gpio;
use std::{sync::mpsc::Sender, thread, time::Duration};


pub enum ButtonToMain {
    Pressed,
}

pub fn poll_button(tx: Sender<ButtonToMain>){
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let pin_number = 17;
    let pin = gpio
        .get(pin_number)
        .expect("Failed to get GPIO pin")
        .into_input_pulldown();
    loop {
        if pin.is_high() {
            thread::sleep(Duration::from_millis(5));
            if pin.is_high() {
                println!("Button Pressed!");
                tx.send(ButtonToMain::Pressed).unwrap();
                thread::sleep(Duration::from_millis(1000));
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
