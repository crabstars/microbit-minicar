# KS4036F + micro:bit v2 Rust Notes (MiniCar Extension Compatible)

This project is aligned with the MakeCode extension:

- `https://github.com/keyestudio2019/MiniCar`

The extension controls the car mainly through I2C at address `0x30`.

## 1) Motor Control

I2C address:

- `0x30`

Registers used by the extension:

- Motor A (M1): `0x01`, `0x02`
- Motor B (M2): `0x04`, `0x03`

Behavior from extension source:

- `motor(M1, Forward, s)` => `reg 0x02 = s`, `reg 0x01 = 0`
- `motor(M1, Backward, s)` => `reg 0x01 = s`, `reg 0x02 = 0`
- `motor(M2, Forward, s)` => `reg 0x03 = s`, `reg 0x04 = 0`
- `motor(M2, Backward, s)` => `reg 0x04 = s`, `reg 0x03 = 0`

Stop both motors:

- write `0` to `0x01`, `0x02`, `0x03`, `0x04`

## 2) Front RGB LEDs

The front LEDs are controlled by I2C registers (active-low style in this extension: `255` is off, smaller values increase brightness).

Right LED channels:

- Red: `0x08`
- Green: `0x07`
- Blue: `0x06`

Left LED channels:

- Red: `0x09`
- Green: `0x0A`
- Blue: `0x05`

Examples:

- Turn all front LED channels off: set `0x05..0x0A` to `255` (matching extension behavior)
- Pure red right LED: `0x08 = 0`, `0x07 = 255`, `0x06 = 255`

## 3) Ultrasonic Sensor

From extension:

- Trig pin: `P14`
- Echo pin: `P15`

Distance formula used:

- `distance_cm = round(pulse_us / 58)`

Important:

- If you also try direct GPIO motor control on `P14/P15`, that conflicts with ultrasonic usage.

## 4) Line Tracking Sensor

From extension:

- Left track sensor: `P12`
- Right track sensor: `P13`
- Both pins are configured with pull-up.

Return value:

- `val = (read(P12) << 0) | (read(P13) << 1)`
- Possible values: `0`, `1`, `2`, `3`

Typical interpretation (common IR line modules, but verify on your board):

- `0`: both detect line
- `1`: left only
- `2`: right only
- `3`: neither detects line

Because module polarity can differ, confirm by printing values while placing sensors over black/white.

## 5) Photoresistance (LDR) Sensors

From extension:

- Left LDR: `P1` (analog)
- Right LDR: `P0` (analog)

Read range:

- `0..1023` (`analogReadPin`)

About black/dark detection:

- There is no fixed universal threshold.
- Use calibration on your board/environment (read values in dark vs bright and choose a midpoint threshold).

## 6) Servo

From extension (`set servo to angle x`):

- Servo pin: `P2`
- Angle range: `0..180`
- Command style: `servoWritePin(P2, angle)`

About your question "what is the last even/event?":

- There is no event in that block.
- It is a direct command: set servo on `P2` to the requested angle immediately.

## 7) Minimal Rust I2C Helper Pattern

```rust
fn write_reg(i2c: &mut Twim<microbit::pac::TWIM0>, reg: u8, value: u8) {
    let _ = i2c.write(0x30, &[reg, value]);
}
```

Use this helper for motor/LED registers above.
