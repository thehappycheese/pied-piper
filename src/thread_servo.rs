use std::{
    sync::{mpsc::{Receiver, RecvTimeoutError}, Arc},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

use crate::{
    config::PiperConfig,
    pca9685::{ServoAction::*, ServoInstruction, ServoNumber::*, PCA9685},
};


/// Open and close the door, coasting at the endpoints.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlternatingSettings {
    /// Delay while door is open
    pub open_pause_seconds: f32,
    
    /// Delay while door is closed
    pub closed_pause_seconds: f32,
    
    /// Time for which to send signal. Only needs to be very small,
    /// since most servos will drive to the last known destination
    /// if they stop receiving PWM signal rather than immediately switching to coast.
    pub drive_time_seconds: f32,
}
impl Default for AlternatingSettings {
    fn default() -> Self {
        AlternatingSettings {
            open_pause_seconds: 2.0,
            closed_pause_seconds: 3.0,
            drive_time_seconds: 0.1,
        }
    }
}

#[derive(Debug)]
pub enum MainToServo {
    Alternate(AlternatingSettings),
    //Neutral,
    Close,
    Open,
    Coast,
}

struct AlternatingState {
    settings: AlternatingSettings,
    last_toggle: Instant,
    is_open: bool,
}

enum State {
    Alternating(AlternatingState),
    Polling,
}

pub fn run_servos(rx: Receiver<MainToServo>, config:Arc<PiperConfig>,) {
    let mut p = PCA9685::new(0x40, 1).unwrap();
    let mut state = State::Polling;
    println!("Servo Thread started!");
    loop {
        let servo_instruction = rx.recv_timeout(Duration::from_millis(10));
        match &servo_instruction {
            Err(RecvTimeoutError::Timeout) => {}
            instruction => println!("Got message {:?}", instruction),
        }
        match servo_instruction {
            Ok(MainToServo::Alternate(settings)) => {
                p.send(ServoInstruction {
                    servo_number: S0,
                    action: Position (config.open_fraction),
                })
                .unwrap();
                state = State::Alternating(AlternatingState {
                    settings,
                    last_toggle: Instant::now(),
                    is_open: true,
                });
            }
            // Ok(MainToServo::Neutral) => {
            //     p.send(ServoInstruction {
            //         servo_number: S0,
            //         action: Position {
            //             value: 0.5,
            //             invert: config.invert,
            //         },
            //     })
            //     .unwrap();
            //     state = State::Polling;
            // }
            Ok(MainToServo::Coast) => {
                p.send(ServoInstruction {
                    servo_number: S0,
                    action: Coast,
                })
                .unwrap();
                state = State::Polling;
            }
            Ok(MainToServo::Close) => {
                p.send(ServoInstruction {
                    servo_number: S0,
                    action: Position (config.closed_fraction),
                })
                .unwrap();
                state = State::Polling;
            }
            Ok(MainToServo::Open) => {
                p.send(ServoInstruction {
                    servo_number: S0,
                    action: Position (config.open_fraction)
                })
                .unwrap();
                state = State::Polling;
            }
            Err(RecvTimeoutError::Timeout) => {
                match &mut state {
                    State::Alternating(AlternatingState {
                        settings: AlternatingSettings{
                            open_pause_seconds,
                            closed_pause_seconds,
                            drive_time_seconds
                        },
                        last_toggle,
                        is_open,
                    }) => {
                        let now = Instant::now();
                        let duration = if *is_open {
                            Duration::from_secs_f32(*open_pause_seconds)
                        } else {
                            Duration::from_secs_f32(*closed_pause_seconds)
                        };
                        if now.duration_since(*last_toggle) > duration {
                            // Toggle servo position
                            *is_open = !*is_open;
                            let fraction = if *is_open {
                                config.open_fraction
                            } else {
                                config.closed_fraction
                            };
                            p.send(ServoInstruction {
                                servo_number: S0,
                                action: Position (fraction),
                            })
                            .unwrap();
                            *last_toggle = now;
                        } else if now.duration_since(*last_toggle) > Duration::from_secs_f32(*drive_time_seconds) {
                            p.send(ServoInstruction{
                                servo_number: S0,
                                action: Coast
                            }).unwrap();
                        }
                    }
                    _ => {}
                }
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    println!("Servo thread exited main loop and finished");
}
