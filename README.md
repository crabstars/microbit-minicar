# microbit-minicar

Small Rust project for the Keyestudio MiniCar with a BBC micro:bit v2.
The project is based on the MakeCode MiniCar extension:

- <https://github.com/keyestudio2019/MiniCar>

## Features

- RGB LEDs
- motor control
- line tracking sensors
- ultrasonic distance sensor
- lcd1602 - adjust the contrast on the back if you don't see any text

## Examples

- `led_color_set`: cycles through the LED colors
- `motor`: drives both motors forward and backward
- `line_tracking`: reads the line sensors and prints the state over RTT
- `ultra`: measures distance and changes the LED color based on the result
- `lcd1602`: writes a text on the screen

Build one example at a time:

```bash
cargo build --example led_color_set
cargo build --example motor
cargo build --example line_tracking
cargo build --example ultra
```

Build the default app:

```bash
cargo build
```

## Target

This project builds for:

```text
thumbv7em-none-eabihf
```

That target is already configured in `.cargo/config.toml`.

If you do not have it installed yet:

```bash
rustup target add thumbv7em-none-eabihf
```

## Flashing

This repo includes an `Embed.toml`, so one simple way to flash is with `cargo-embed`:

```bash
cargo embed --example motor
```

Many examples print debug output over RTT.
