use crate::{
    drivers::kb::{driver::KbDriver, input::KbDriverInput, state::KbDriverState},
    errors::A2PiError,
};
use mio_serial::SerialStream;
use std::sync::Arc;

use parking_lot::FairMutex;

#[derive(Debug, Clone)]
pub enum State {
    Start,
    Run,
    // Reset,
    // Stop,
}

pub struct A2PiState {
    pub state: State,
    pub kb_driver: KbDriver,
    pub kb_driver_state: Arc<FairMutex<KbDriverState>>,
}

impl Clone for A2PiState {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            kb_driver: self.kb_driver.clone(),
            kb_driver_state: self.kb_driver_state.clone(),
        }
    }
}

impl A2PiState {
    pub fn new() -> A2PiState {
        A2PiState {
            state: State::Start,
            kb_driver: KbDriver::init(None),
            kb_driver_state: Arc::new(FairMutex::new(KbDriverState::reset())),
        }
    }
    pub fn handler(&mut self, conn: &mut SerialStream, payload: &[u8]) -> Result<(), A2PiError> {
        println!("{:?}, {:02X?}", self.state, payload);

        match self.state {
            State::Start => {
                let handshake = self.kb_driver.handshake(conn, payload);
                match handshake {
                    Ok(_) => {
                        self.state = State::Run;
                        println!("Apple II connected!");
                    }
                    Err(e) => {
                        println!("{:02X?} {:?}", payload, e);
                        let _may_fail = self.kb_driver.reset(conn);
                    }
                }
            }
            State::Run => {
                if payload.len() % 3 != 0 || payload[0] == 0x98 {
                    println!("malformed kb input!!!");
                    if payload[0] != 0x98 {
                        self.kb_driver.reset_device();
                    }
                    return Ok(());
                }

                // payload.len() may exceed 3 indicating an multi key presses
                // so we should chunk each 3 pair and iter over each key press
                let chunks: Vec<&[u8]> = payload.chunks(3).collect();
                for payload_chunk in chunks {
                    let kb_input = KbDriverInput::from_apple_ii(payload_chunk, &|scan_code| {
                        self.kb_driver.clone().lookup_scan_code(scan_code)
                    });
                    if let Err(e) = kb_input {
                        match e {
                            A2PiError::InvalidKBPayload => {
                                self.state = State::Start;
                            }
                            A2PiError::InvalidKBInput => {
                                println!("invalid kb input!!!");
                            }
                            A2PiError::InvalidKBModifier => {
                                println!("invalid kb modifier!!!");
                            }
                            _ => {}
                        }
                        self.kb_driver.reset_device();
                        return Ok(());
                    }
                    {
                        let guard = self.kb_driver_state.try_lock();
                        if let Some(mut kb_driver_state) = guard {
                            let kb_inp = kb_input.unwrap().clone().unwrap();

                            (*kb_driver_state).process_input(kb_inp.clone());

                            if !kb_driver_state.chained_key_inputs.is_empty() {
                                //
                                self.kb_driver
                                    .emit_to_device((*kb_driver_state).clone(), kb_inp);
                            } else {
                                self.kb_driver.reset_device();
                            }
                        } else {
                            println!("kb_driver_state is locked. unable to handle kb input!!!");
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
