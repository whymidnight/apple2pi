# REPL readme

## commands

```
reset </dev/ttyUSB0>   | attempt to reset/resync the `apple2pi` client.
monitor </dev/ttyUSB0> | monitor ingress scan codes and display their associated key press event.
```

### `reset <SERIAL_DEVICE>`

this is useful to commandeer should `a2pi-rs` become bugged.

### `monitor <SERIAL_DEVICE>`

this is useful to identify key press events.
