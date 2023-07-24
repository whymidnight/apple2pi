//! Simple example that echoes received serial traffic to stdout
extern crate mio;
extern crate mio_serial;

use hex::FromHex;
use mio::{Events, Interest, Poll, Token};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use signal_hook::consts::TERM_SIGNALS;

use signal_hook::flag;

use std::env;
use std::io;
use std::io::Read;
use std::io::Write;
use std::str;

mod drivers;
mod errors;
mod state;

use state::A2PiState;

use mio_serial::SerialPortBuilderExt;

const SERIAL_TOKEN: Token = Token(0);

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";

const DEFAULT_BAUD: u32 = 115200;

pub fn main() -> io::Result<()> {
    let term_now = Arc::new(AtomicBool::new(false));

    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    let mut args = env::args();
    let path = args.nth(1).unwrap_or(DEFAULT_TTY.into());

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1);

    println!("Opening {} at {},8N1", path, DEFAULT_BAUD);
    let mut conn = mio_serial::new(path, DEFAULT_BAUD).open_native_async()?;

    // #[cfg(unix)]
    // let mut rx = mio_serial::TTYPort::open(&builder)?;
    // #[cfg(windows)]
    // let mut rx = mio_serial::COMPort::open(&builder)?;

    poll.registry()
        .register(&mut conn, SERIAL_TOKEN, Interest::READABLE)
        .unwrap();

    let mut buf = [0u8; 1024];
    let ack = <[u8; 1]>::from_hex("80").unwrap();
    let ack_write = conn.write(&ack);
    if let Err(ack_write_err) = ack_write {
        panic!("unable to init! {:?}", ack_write_err);
    }

    let mut kb_driver = A2PiState::new();

    while !term_now.load(Ordering::Relaxed) {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERIAL_TOKEN => loop {
                    // In this loop we receive all packets queued for the socket.
                    match conn.read(&mut buf) {
                        Ok(count) => {
                            let handler = kb_driver.handler(&mut conn, &buf[..count]);

                            if let Err(handler_err) = handler {
                                println!("{:?}", handler_err);
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                _ => {
                    println!("unforeseen")
                }
            }
        }
    }

    kb_driver.shutdown();
    println!("Shutdown successful!");

    Ok(())
}
