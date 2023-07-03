# readme

## obligatory disclosure

i wrote this on like 6 hours of sleep spanning 3 days.

## technical info

- uses [mio-serial]() to interface and communicate with `apple2pi` at 115200
  baud.
- uses [enigo]() for simulating keyboard events to the host OS.

### macro/remapping support

macro's are flagship feature and can be orchestrated from within
`./src/drivers/kb/vdev/key_codex.rs` until i roll in `fn from_spec()` support;
which will allow on-the-fly macro reconfiguration via _spec.json files_.

#### layers

currently the following layers are supported:

```
Modifier         | Code
-----------------------
Open Apple:        0x40
Closed Apple:      0x80
Open Closed Apple: 0xC0
```

### cross-platform via **enigo**

_enigo_ appears to be cross-platform - supporting X11 (*nix), macOS, and Windows
operating systems. such macOS and Windows support has not been confirmed but
_should supposedly work_.
