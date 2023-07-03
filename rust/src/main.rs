//! Simple example that echoes received serial traffic to stdout

use errors::A2PiError;
use hex::FromHex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio_serial::{SerialPortBuilderExt, SerialStream};

// This is just a collection of ints that represent kill signals.
// More specifically, they are the common kill signals used to
// terminate a program
// You can do println!("{:?}", TERM_SIGNALS) to see them
// They are just SIGINT(2), SIGTERM(15) and SIGQUIT(3)
use signal_hook::consts::TERM_SIGNALS;

// Module that sets boolean flags when kill signal is received
use signal_hook::flag;

use futures::stream::StreamExt;
use std::env;
use std::io;
use std::io::Write;
use std::str;
use tokio_util::codec::{Decoder, Encoder};

use bytes::BytesMut;

mod drivers;
mod errors;
mod state;

use state::A2PiState;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";

struct LineCodec {
    pub a2pi: A2PiState,
}

impl LineCodec {
    pub fn init() -> LineCodec {
        let a2pi = A2PiState::new();

        Self { a2pi }
    }
}

impl Decoder for LineCodec {
    type Item = Result<(), A2PiError>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut conn = tokio_serial::new(DEFAULT_TTY, 115200).open_native_async().unwrap();
        conn.set_exclusive(false).unwrap();

        let _handler = self.a2pi.handler(&mut conn, src.to_vec().as_slice());

        src.clear();
        Ok(None)
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
    let term_now = Arc::new(AtomicBool::new(false));

    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    let mut args = env::args();
    let _path = args.nth(1).unwrap_or(DEFAULT_TTY.into());

    let mut conn = tokio_serial::new(DEFAULT_TTY, 115200).open_native_async()?;
    conn.set_exclusive(false)?;

    let ack = <[u8; 1]>::from_hex("80").unwrap();
    let ack_write = conn.write(&ack);
    if let Err(ack_write_err) = ack_write {
        panic!("unable to init! {:?}", ack_write_err);
    }

    let codec = LineCodec::init();
    let mut reader = codec.framed(conn);

    while !term_now.load(Ordering::Relaxed) {
        while let Some(buf) = reader.next().await {
            match buf {
                Ok(buf) => {
                    println!("{:?}", buf)
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        /*
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
                            println!("Quitting due to read error: {}", e);
                            return Err(e);
                        }
                    }
                },
                _ => {
                    // This should never happen as we only registered our
                    // `UdpSocket` using the `UDP_SOCKET` token, but if it ever
                    // does we'll log it.
                    println!("Got event for unexpected token: {:?}", event);
                }
            }
        }
        */
    }

    // codec.a2pi.clone().shutdown();
    println!("Shutdown successful!");
    /*
     */
    Ok(())
}
