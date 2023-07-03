# configure keyboard layout

## base layer

can be found in `./a2pi_keymaps/kbmap.json`.

### reprogram (FUTURE TODO)

key down scan codes differ from their ascii hex representation. use
`cargo run --bin repl_keycodes` to tweak `./a2pi_keymaps/kbmap.json` instead.

## additional layers

modifier scan codes are never sent as bare without a character scan code.
**always** accompanied with a character scan code.

### 1) Ctrl (0x00)

32 ctrl + (key) combinations

### 2) Open Apple (0x40)

?? Open Apple + (key) combinations

### 3) Closed Apple (0x80)

?? Closed Apple + (key) combinations

### 4) Open + Closed Apple (0xC0)

??

### 5) Ctrl + Open Apple (0x00 + 0x40)

??

### 6) Ctrl + Closed Apple (0x00 + 0x80)

??
