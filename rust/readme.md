# readme

this is a rust implementation of the keyboard driver for apple2pi.

## motivaton

i love QMK, rust, and german keyboards. so with an aIIe ISO layout, i found the
original C architecture of the server pretty cumbersone to bake in macros and
remapping was atrocious in `kbmap.c`.

seems like keycodes >64 are dropped. i believe due to ASCII being a 64 character
set? not a vintage computing geek. this caused evdev to not reciprocate
`KEY_102ND` up to X - which is the (extra) <|> key next to left shift. i also
belive that Apple used an extended ASCII character set on PAL II's to accomodate
the particular language like umlauts found on the german keyboard.

anyway, i am not an avid C programmer. i need macro and remapping support for my
work. choosing rust should make maintaining this more feasible.

## **THE KB PROTOCOL** (reverse engineered)

```
---
> |C: Rust| relationships:
* a2fd: conn
---

> init
>> S0
- open a uinput device
- set key repeat

>> S1
- open conn to serial endpoint (/dev/ttyUSB0)

> handshake
>> ACK
- emit `0x80` packet over conn (request re-sync if Apple II already running)

>> SYN
while Some(rx_packet) = conn.read().await {
    match rx_packet.data {
        syn if rx_packet.data == 0x80 => { /* receive sync */
            println!("a2pid: Connected.");
            conn.write(0x81).await;  /* acknowledge */
        }
        bad_request if rx_packet.data == 0x9F => { /* bad request from Apple II */
            // TCIFLUSH ???
        }
        _ => {
            panic!("a2pi: Bad Sync ACK")
        }
    }
}

>> ACK
- blocking read until 3 chars received

> LOOP
while Some(rx_packet) = conn.read().await {
    match rx_packet.data {
        syn if rx_packet.data == 0x80 => { /* receive sync */
            println!("a2pid: Connected.");
            conn.write(0x81).await;  /* acknowledge */
        }
        bad_request if rx_packet.data == 0x9F => { /* bad request from Apple II */
            // TCIFLUSH ???
        }
        _ => {
            panic!("a2pi: Bad Sync ACK")
        }
    }
}
```
