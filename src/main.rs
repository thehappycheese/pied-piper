use std::{
    sync::{mpsc::{channel, Receiver, RecvTimeoutError, Sender}, Arc},
    thread,
    time::Duration,
};

mod pca9685;

mod thread_led;
use thread_led::run_leds;

mod thread_button;
use thread_button::{poll_button, ButtonToMain};

mod thread_music;
use thread_music::{play_music, MainToMusic, MusicToMain};

mod thread_servo;
use thread_servo::{run_servos, MainToServo};

mod thread_jiggler;
use thread_jiggler::keep_speaker_awake;

mod config;
use config::PiperConfig;

fn main() {
    
    let config = Arc::new(PiperConfig::load_from_file("config.json"));

    let (tx_button, rx_button) = channel();
    let (main_send_music, rx_music) = channel();
    let (tx_music, main_recv_music) = channel();
    let (tx_servo, rx_servo) = channel();
    
    let _thread_run_leds = thread::spawn(run_leds);
    let _thread_noise = thread::spawn(keep_speaker_awake);

    let _thread_play_music = thread::spawn({
        let config = config.clone();
        move || play_music(main_send_music, main_recv_music, config)
    });
    
    let _thread_poll_button = thread::spawn({
        let config = config.clone();
        move || poll_button(tx_button, config)
    });
    
    let _thread_run_servos = thread::spawn({
        let config = config.clone();
        move || run_servos(rx_servo, config)
    });

    // close door and coast 
    tx_servo.send(MainToServo::Close).unwrap();
    thread::sleep(Duration::from_secs_f32(1.0));
    tx_servo.send(MainToServo::Coast).unwrap();

    // wait for the button to be pressed
    loop{
        thread::sleep(Duration::from_secs_f32(0.2));
        let events:Vec<ButtonToMain> = rx_button.try_iter().collect();
        match events.first() {
            Some(ButtonToMain::Pressed)=>{
                // execute performance
                // # TODO prevent immediate re-execution
                performance(tx_music.clone(), &rx_music, tx_servo.clone(), &config);
            }
            None=>{
            }
        }
        for music_event in rx_music.try_iter() {
            println!("MAIN THREAD IDLE: DISCARDING MESSAGE FROM MUSIC THREAD {music_event:?}")
        }
    }
}


fn performance(
    tx_music:Sender<MainToMusic>,
    rx_music:&Receiver<MusicToMain>,
    tx_servo:Sender<MainToServo>,
    config:&PiperConfig
){
    tx_music.send(MainToMusic::Play).unwrap();
    loop{
        match rx_music.recv_timeout(Duration::from_millis(100)) {
            Ok(MusicToMain::StartedPlaying)=>{
                tx_servo.send(MainToServo::Alternate(config.alternation_settings.clone())).unwrap();
            }
            Ok(MusicToMain::StoppedPlaying)=>{
                tx_servo.send(MainToServo::Close).unwrap();
                thread::sleep(Duration::from_secs_f32(1.0));
                tx_servo.send(MainToServo::Coast).unwrap();
                break;
            }
            Ok(MusicToMain::MusicError(error_type))=>{
                println!("Could not play Music due to error type: {error_type:?}");
                break;
            }
            Ok(MusicToMain::GotOutputDevice)=>{
                println!("PERFORMANCE: MUSIC THREAD GOT OUTPUT DEVICE");
                continue;
            }
            Err(RecvTimeoutError::Timeout)=>continue,
            Err(RecvTimeoutError::Disconnected)=>break,
        }
    }
}
