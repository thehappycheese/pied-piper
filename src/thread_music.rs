use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::thread;
use std::{
    sync::mpsc::{Receiver, RecvTimeoutError, Sender},
    time::Duration,
};

use rodio::{
    source::{SamplesConverter, Source},
    Decoder, OutputStream, Sink,
};

use crate::config::PiperConfig;

#[derive(Debug)]
pub enum MusicErrorType {
    NoOutputDevice,
    UnableToOpenFile,
    DecodeError,
    UnknownError,
}

#[derive(Debug)]
pub enum MusicToMain {
    StartedPlaying,
    StoppedPlaying,
    GotOutputDevice,
    MusicError(MusicErrorType),
}
pub enum MainToMusic {
    Play,
}

enum MusicState {
    Playing(
        // file:File,
        //source:SamplesConverter<Decoder<BufReader<File>>, f32>,
        //sink:Sink,
        Sink,
    ),
    Stopped,
}

pub fn play_music(
    tx: Sender<MusicToMain>,
    rx: Receiver<MainToMusic>,
    config:Arc<PiperConfig>,
) {
    let mut state = MusicState::Stopped;
    // Try to secure an output stream

    let x;
    loop{
        println!("MUSIC THREAD: WILL TRY TO GET AUDIO DEVICE NOW:");
        match OutputStream::try_default() {
            Ok(os) => {
                x=os;
                println!("MUSIC THREAD: GOT AUDIO DEVICE NOW:");
                tx.send(MusicToMain::GotOutputDevice).unwrap();
                break;
            },
            Err(_) => {
                println!("MUSIC THREAD: FAILED TO GET AUDIO DEVICE");
                tx.send(MusicToMain::MusicError(MusicErrorType::NoOutputDevice)).unwrap();
                let _junk:Vec<_> = rx.try_iter().collect();
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };
    }
    let (_stream, stream_handle) = x;
    loop {
        match rx.recv_timeout(Duration::from_millis(500)) {
            Err(RecvTimeoutError::Disconnected) => {
                break;
            }
            Err(RecvTimeoutError::Timeout) => match &state {
                MusicState::Stopped => {}
                MusicState::Playing(sink) => {
                    if sink.empty() {
                        state = MusicState::Stopped;
                        println!("Music Finished Playing");
                        tx.send(MusicToMain::StoppedPlaying).unwrap();
                    }
                }
            },
            Ok(MainToMusic::Play) => match state {
                MusicState::Playing(_) => {
                    // Do nothing
                }
                MusicState::Stopped => {
                    // Load a sound from a file, using a path relative to Cargo.toml
                    // let file = match File::open("../music/pied piper 3.mp3"){
                    let file = match File::open(config.music_file_location.clone()) {
                        Ok(f) => f,
                        Err(_) => {
                            tx.send(MusicToMain::MusicError(MusicErrorType::UnableToOpenFile))
                                .unwrap();
                            continue;
                        }
                    };

                    let buf_reader = BufReader::new(file);

                    let decoder = match Decoder::new(buf_reader) {
                        Ok(d) => d,
                        Err(_) => {
                            tx.send(MusicToMain::MusicError(MusicErrorType::DecodeError))
                                .unwrap();
                            continue;
                        }
                    };

                    let source: SamplesConverter<Decoder<BufReader<File>>, f32> =
                        decoder.convert_samples();

                    let sink = match Sink::try_new(&stream_handle) {
                        Ok(s) => s,
                        Err(_) => {
                            tx.send(MusicToMain::MusicError(MusicErrorType::UnknownError))
                                .unwrap();
                            continue;
                        }
                    };
                    sink.append(source);
                    sink.play();
                    state = MusicState::Playing(sink);
                    tx.send(MusicToMain::StartedPlaying).unwrap();
                    println!("Music Started Playing");
                }
            },
        }
    }
    println!("Music Thread Exited Loop and Completed")
}
