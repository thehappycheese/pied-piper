use rppal::gpio::Gpio;
use std::{sync::mpsc, thread, time::Duration};
use std::fs::File;
use std::io::BufReader;

use rodio::{source::{SamplesConverter, Source}, Decoder, OutputStream, Sink};

mod led;
use led::run_leds;

mod pca9685;
use pca9685::PCA9685;

enum MusicControl {
    Play,
}

fn main(){
    let (tx, rx) = mpsc::channel();

    let threads: Vec<std::thread::JoinHandle<()>> = vec![
        thread::spawn(||{
            run_leds();
            ()
        }),
        thread::spawn(move || {
            play_music(rx);
            ()
        }),
        thread::spawn(move || {
            poll_gpio_pin(tx);
            ()
        }),
        thread::spawn(||{
            run_i2c();
            ()
        }),
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

fn play_music(rx: mpsc::Receiver<MusicControl>) {
    // Get an output stream handle to the default physical sound device
    loop{
        if let Ok(_) = rx.recv() {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open("../music/pied piper 3.mp3").unwrap());
            // Decode that sound file into a source
            let source:SamplesConverter<Decoder<BufReader<File>>, f32> = Decoder::new(file).unwrap().convert_samples();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(source);
            sink.pause();
            sink.play();
            sink.sleep_until_end();
        }
    }
}

fn poll_gpio_pin(tx: mpsc::Sender<MusicControl>){
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let pin_number = 17;
    let pin = gpio
        .get(pin_number)
        .expect("Failed to get GPIO pin")
        .into_input_pulldown();

    loop {
        if pin.is_high() {
            println!("Button Pressed!");
            tx.send(MusicControl::Play);
        }
        thread::sleep(Duration::from_millis(200));
    }
}
