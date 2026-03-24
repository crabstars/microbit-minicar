# microbit-minicar

Small Rust library and example project for the Keyestudio MiniCar with a BBC micro:bit v2.

The code is meant to stay easy to read and easy to reuse in other embedded Rust projects.
It is based on the MakeCode MiniCar extension:

- <https://github.com/keyestudio2019/MiniCar>

## What this crate gives you

- motor control
- RGB LED control
- line tracking sensor reading
- ultrasonic distance measurement
- LCD1602 helper for I2C displays

The reusable library lives in `src/lib.rs`.
The board setup for the micro:bit stays in `examples/`.

## Use it as a library

From GitHub:

```toml
[dependencies]
microbit-minicar = { git = "https://github.com/crabstars/microbit-minicar" }
```

In code:

```rust
use microbit_minicar::led::{self, LedColor, LedRgb};
use microbit_minicar::motor::{self, Direction, Motor};

fn demo<I2C>(i2c: &mut I2C) -> Result<(), I2C::Error>
where
    I2C: embedded_hal::i2c::I2c,
{
    motor::set(i2c, 90, Motor::A, Direction::Forward)?;
    led::set_color(i2c, LedRgb::Led1, LedColor::Green)?;
    Ok(())
}
```

The library uses `embedded-hal` traits, so it is not locked to one exact board type.

## Examples

- `led_color_set`: cycles through the LED colors
- `motor`: drives both motors forward and backward
- `line_tracking`: reads the line sensors and prints the state over RTT
- `ultra`: measures distance and changes the LED color based on the result
- `lcd1602`: writes text to the display

Build one example at a time:

```bash
cargo build --example led_color_set
cargo build --example motor
cargo build --example line_tracking
cargo build --example ultra
cargo build --example lcd1602
```

Build the library:

```bash
cargo build
```

## Target

This repo is configured for:

```text
thumbv7em-none-eabihf
```

Install it if needed:

```bash
rustup target add thumbv7em-none-eabihf
```

## Flashing

This repo includes an `Embed.toml`, so a simple flash command is:

```bash
cargo embed --example motor
```

Many examples print debug output over RTT.

## Notes

- `docs/pwm.md` explains the PWM setup in simple terms
- if the LCD stays blank, adjust the contrast trim pot on the module
