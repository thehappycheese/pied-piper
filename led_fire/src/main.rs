use std::{
    sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender},
    thread,
    time::Duration,
};

mod pca9685;

mod thread_led;
use thread_led::run_leds;

mod thread_button;
use thread_button::{poll_button, ButtonToMain};

mod thread_music;
use thread_music::{play_music, MainToMusic, MusicErrorType, MusicToMain};

mod thread_servo;
use thread_servo::{run_servos, AlternatingSettings, MainToServo};

fn main() {
    let (tx_button, rx_button) = channel();
    let (main_send_music, rx_music) = channel();
    let (tx_music, main_recv_music) = channel();
    let (tx_servo, rx_servo) = channel();

    let invert = true;
    
    let thread_run_leds = thread::spawn(|| run_leds());
    let thread_play_music = thread::spawn(move || play_music(
        main_send_music,
        main_recv_music,
    ));
    let thread_poll_button = thread::spawn(move || poll_button(tx_button));
    let thread_run_servos = thread::spawn(move || run_servos(rx_servo, invert));

    // wait for the button to be pressed
    loop{
        match rx_button.recv_timeout(Duration::from_millis(500)){
            Ok(ButtonToMain::Pressed)=>{
                // execute performance
                // # TODO prevent immediate re-execution
                performance(tx_music.clone(), &rx_music, tx_servo.clone());
                
            },
            Err(RecvTimeoutError::Disconnected)=>{
                // button died. Bail out.
                break;
            }
            Err(RecvTimeoutError::Timeout)=>{
                // loomp
            }
        }
    }
}


fn performance(
    tx_music:Sender<MainToMusic>,
    rx_music:&Receiver<MusicToMain>,
    tx_servo:Sender<MainToServo>
){
    tx_music.send(MainToMusic::Play).unwrap();
    loop{
        match rx_music.recv_timeout(Duration::from_millis(100)) {
            Ok(MusicToMain::StartedPlaying)=>{
                tx_servo.send(MainToServo::Alternate(AlternatingSettings{
                    open_fraction:1.0,
                    closed_fraction:0.0,
                    open_pause_seconds:2,
                    closed_pause_seconds:3
                })).unwrap();
            }
            Ok(MusicToMain::StoppedPlaying)=>{
                tx_servo.send(MainToServo::Close).unwrap();
                break
            }
            Ok(MusicToMain::MusicError(_))=>{
                break
            }
            Err(RecvTimeoutError::Timeout)=>continue,
            Err(RecvTimeoutError::Disconnected)=>break,
        }
    }
}
