use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use std::{thread, time::Duration};
use std::fs::File;
use std::io::BufReader;

use rodio::{source::{SamplesConverter, Source}, Decoder, OutputStream, Sink};

mod led;
use led::run_leds;

mod pca9685;
use pca9685::PCA9685;

fn main(){

    let threads = vec![
        thread::spawn(||run_leds()),
        thread::spawn(||play_music()),
        thread::spawn(||poll_gpio_pin()),
        thread::spawn(||run_i2c()),
    ];
    for thread in threads {
        let _ = thread.join();
    }
}

fn run_i2c(){
    let mut p = PCA9685::new(0x40, 1).unwrap();
    loop {
        p.set_servo(0, 0.7).unwrap();
        thread::sleep(Duration::from_millis(2000));
        p.set_servo(0, 0.0).unwrap();
        thread::sleep(Duration::from_millis(2500));
    }
}

fn play_music() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("../music/pied piper 3.mp3").unwrap());
    // Decode that sound file into a source
    let source:SamplesConverter<Decoder<BufReader<File>>, f32> = Decoder::new(file).unwrap().convert_samples();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

fn poll_gpio_pin(){
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let pin_number = 17;
    let pin = gpio
        .get(pin_number)
        .expect("Failed to get GPIO pin")
        .into_input();
    loop {
        let level = pin.read();
        //println!("Motion detected: {:?}", level == rppal::gpio::Level::High);
        thread::sleep(Duration::from_millis(200));
    }
}
