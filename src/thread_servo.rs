use std::{
    sync::{mpsc::{Receiver, RecvTimeoutError}, Arc},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

use crate::{
    config::PiperConfig,
    pca9685::{ServoAction::*, ServoInstruction, ServoNumber::*, PCA9685},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlternatingSettings {
    pub open_fraction: f32,
    pub closed_fraction: f32,
    pub open_pause_seconds: f32,
    pub closed_pause_seconds: f32,
}
impl Default for AlternatingSettings {
    fn default() -> Self {
        AlternatingSettings {
            open_fraction: 1.0,
            closed_fraction: 0.0,
            open_pause_seconds: 2.0,
            closed_pause_seconds: 3.0,
        }
    }
}

#[derive(Debug)]
pub enum MainToServo {
    Alternate(AlternatingSettings),
    Neutral,
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
                    action: Position {
                        value: 0.0,
                        invert: config.invert,
                    },
                })
                .unwrap();
                state = State::Alternating(AlternatingState {
                    settings,
                    last_toggle: Instant::now(),
                    is_open: true,
                });
            }
            Ok(MainToServo::Neutral) => {
                p.send(ServoInstruction {
                    servo_number: S0,
                    action: Position {
                        value: 0.5,
                        invert: config.invert,
                    },
                })
                .unwrap();
                state = State::Polling;
            }
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
                    action: Position {
                        value: 0.0,
                        invert: config.invert,
                    },
                })
                .unwrap();
                state = State::Polling;
            }
            Ok(MainToServo::Open) => {
                p.send(ServoInstruction {
                    servo_number: S0,
                    action: Position {
                        value: 1.0,
                        invert: config.invert,
                    },
                })
                .unwrap();
                state = State::Polling;
            }
            Err(RecvTimeoutError::Timeout) => {
                match &mut state {
                    State::Alternating(AlternatingState {
                        settings,
                        last_toggle,
                        is_open,
                    }) => {
                        let now = Instant::now();
                        let duration = if *is_open {
                            Duration::from_secs_f32(settings.open_pause_seconds)
                        } else {
                            Duration::from_secs_f32(settings.closed_pause_seconds)
                        };
                        if now.duration_since(*last_toggle) >= duration {
                            // Toggle servo position
                            *is_open = !*is_open;
                            let fraction = if *is_open {
                                settings.open_fraction
                            } else {
                                settings.closed_fraction
                            };
                            p.send(ServoInstruction {
                                servo_number: S0,
                                action: Position {
                                    value: fraction,
                                    invert: config.invert,
                                },
                            })
                            .unwrap();
                            *last_toggle = now;
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
