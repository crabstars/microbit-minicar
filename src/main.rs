#![no_std]
#![no_main]

use core::hint::spin_loop;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::{
    board::Board,
    hal::{
        Timer,
        gpio::Level,
        twim::{self, Twim},
    },
};
use panic_halt as _;

const I2C_ADDR: u8 = 0x30;
const ULTRA_MAX_PULSE_US: u32 = 35_000;

fn elapsed_us<U>(clock: &Timer<microbit::pac::TIMER1, U>, start: u32) -> u32 {
    clock.read().wrapping_sub(start)
}

fn pulse_in_high<U>(
    clock: &Timer<microbit::pac::TIMER1, U>,
    echo: &mut impl InputPin,
    timeout_us: u32,
) -> u32 {
    let wait_start = clock.read();
    while echo.is_low().ok().unwrap_or(false) {
        if elapsed_us(clock, wait_start) >= timeout_us {
            return 0;
        }
        spin_loop();
    }

    let pulse_start = clock.read();
    while echo.is_high().ok().unwrap_or(false) {
        if elapsed_us(clock, pulse_start) >= timeout_us {
            return 0;
        }
        spin_loop();
    }

    elapsed_us(clock, pulse_start)
}

fn ultra<U>(
    timer: &mut Timer<microbit::pac::TIMER0>,
    clock: &Timer<microbit::pac::TIMER1, U>,
    trig: &mut impl OutputPin,
    echo: &mut impl InputPin,
    last_time_us: &mut u32,
) -> u32 {
    let settle_start = clock.read();
    while echo.is_high().ok().unwrap_or(false) {
        if elapsed_us(clock, settle_start) >= ULTRA_MAX_PULSE_US {
            break;
        }
        spin_loop();
    }

    let _ = trig.set_low();
    timer.delay_us(2);
    let _ = trig.set_high();
    timer.delay_us(10);
    let _ = trig.set_low();

    let t = pulse_in_high(clock, echo, ULTRA_MAX_PULSE_US);
    let mut ret = t;

    if ret == 0 && *last_time_us != 0 {
        ret = *last_time_us;
    }
    *last_time_us = t;

    (ret + 29) / 58
}

enum Direction {
    Forward = 1,
    Backward = 2,
}

enum Motor {
    A = 1,
    B = 2,
}

enum LedColor {
    Red = 1,
    Green = 2,
    Blue = 3,
    Cyan = 4,
    Purple = 5,
    White = 6,
    Yellow = 7,
    Black = 8,
}

enum LedRgb {
    Led1 = 1,
    Led2 = 2,
}

#[repr(u8)]
enum RightLED {
    Red = 0x08,
    Green = 0x07,
    Blue = 0x06,
}

#[repr(u8)]
enum LeftLED {
    Red = 0x09,
    Green = 0x0A,
    Blue = 0x05,
}

fn write_reg(i2c: &mut Twim<microbit::pac::TWIM0>, reg: u8, value: u8) {
    let _ = i2c.write(I2C_ADDR, &[reg, value]);
}

fn motor_stop(i2c: &mut Twim<microbit::pac::TWIM0>) {
    write_reg(i2c, 0x01, 0);
    write_reg(i2c, 0x02, 0);
    write_reg(i2c, 0x03, 0);
    write_reg(i2c, 0x04, 0);
}

fn motor(i2c: &mut Twim<microbit::pac::TWIM0>, speed: u8, motorlist: Motor, direction: Direction) {
    match direction {
        Direction::Forward => match motorlist {
            Motor::A => {
                write_reg(i2c, 0x01, 0);
                write_reg(i2c, 0x02, speed);
            }
            Motor::B => {
                write_reg(i2c, 0x03, speed);
                write_reg(i2c, 0x04, 0);
            }
        },
        Direction::Backward => match motorlist {
            Motor::A => {
                write_reg(i2c, 0x01, speed);
                write_reg(i2c, 0x02, 0);
            }
            Motor::B => {
                write_reg(i2c, 0x03, 0);
                write_reg(i2c, 0x04, speed);
            }
        },
    }
}

// NOTE: 0 = ON, 255 = OFF
fn color_to_pwm(c: LedColor) -> (u8, u8, u8) {
    match c {
        LedColor::Red => (0, 255, 255),
        LedColor::Green => (255, 0, 255),
        LedColor::Blue => (255, 255, 0),

        LedColor::Cyan => (255, 0, 0),
        LedColor::Purple => (0, 255, 0),
        LedColor::White => (0, 0, 0),

        LedColor::Yellow => (0, 0, 255),
        LedColor::Black => (255, 255, 255), // OFF
    }
}

fn led_set_color(i2c: &mut Twim<microbit::pac::TWIM0>, led: LedRgb, led_color: LedColor) {
    let (r, g, b) = color_to_pwm(led_color);

    match led {
        LedRgb::Led1 => {
            write_reg(i2c, RightLED::Red as u8, r);
            write_reg(i2c, RightLED::Green as u8, g);
            write_reg(i2c, RightLED::Blue as u8, b);
        }
        LedRgb::Led2 => {
            write_reg(i2c, LeftLED::Red as u8, r);
            write_reg(i2c, LeftLED::Green as u8, g);
            write_reg(i2c, LeftLED::Blue as u8, b);
        }
    }
}

// NOTE: 0 = ON, 255 = OFF
fn led_set_rgb(i2c: &mut Twim<microbit::pac::TWIM0>, led: LedRgb, rgb: (u8, u8, u8)) {
    let (r, g, b) = rgb;

    match led {
        LedRgb::Led1 => {
            write_reg(i2c, RightLED::Red as u8, r);
            write_reg(i2c, RightLED::Green as u8, g);
            write_reg(i2c, RightLED::Blue as u8, b);
        }
        LedRgb::Led2 => {
            write_reg(i2c, LeftLED::Red as u8, r);
            write_reg(i2c, LeftLED::Green as u8, g);
            write_reg(i2c, LeftLED::Blue as u8, b);
        }
    }
}

fn led_disable(i2c: &mut Twim<microbit::pac::TWIM0>) {
    write_reg(i2c, RightLED::Red as u8, 255);
    write_reg(i2c, RightLED::Blue as u8, 255);
    write_reg(i2c, RightLED::Green as u8, 255);
    write_reg(i2c, LeftLED::Red as u8, 255);
    write_reg(i2c, LeftLED::Blue as u8, 255);
    write_reg(i2c, LeftLED::Green as u8, 255);
}
#[entry]
fn main() -> ! {
    // The TWIM instances share the same address space with instances of SPIM, SPIS, SPI, TWIS, and TWI. For example, TWIM0 conflicts with SPIM0, SPIS0, etc.; TWIM1 conflicts with SPIM1, SPIS1, etc. You need to make sure that conflicting instances are disabled before using Twim. Please refer to the product specification for more information (section 15.2 for nRF52832, section 6.1.2 for nRF52840).
    // TWI (“Two-Wire Interface”), TWIM (“Two-Wire Interface Master”) and TWIS (“Two-Wire Interface Slave”).
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut pulse_clock = Timer::periodic(board.TIMER1);
    pulse_clock.start(u32::MAX);

    let mut trig_pin = board.pins.p0_01.into_push_pull_output(Level::Low);
    let mut echo_pin = board.pins.p0_13.into_floating_input();
    let mut ultra_last_time_us = 0_u32;

    // MakeCode uses the external micro:bit I2C bus (P19/P20).
    let mut i2c = Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        twim::Frequency::K100,
    );

    // Safety: always stop first (prevents stale motor state).
    motor_stop(&mut i2c);
    led_disable(&mut i2c);
    // led_show(&mut i2c, LedRgb::Led1, LedColor::Green);
    // led_set_color(&mut i2c, LedRgb::Led2, (255 - 153, 255 - 153, 255));
    // timer.delay_ms(100_u32);
    //
    // // motor(&mut i2c, 50, Direction::Backward);
    // motor(&mut i2c, 50, Motor::A, Direction::Forward);
    // motor(&mut i2c, 100, Motor::B, Direction::Forward);
    // timer.delay_ms(10000_u32);
    //
    // motor_stop(&mut i2c);
    // timer.delay_ms(2000_u32);

    loop {
        let distance_cm = ultra(
            &mut timer,
            &pulse_clock,
            &mut trig_pin,
            &mut echo_pin,
            &mut ultra_last_time_us,
        );

        if distance_cm > 0 && distance_cm <= 10 {
            led_set_color(&mut i2c, LedRgb::Led1, LedColor::Red);
            led_set_color(&mut i2c, LedRgb::Led2, LedColor::Red);
        } else {
            led_set_color(&mut i2c, LedRgb::Led1, LedColor::Green);
            led_set_color(&mut i2c, LedRgb::Led2, LedColor::Green);
        }

        timer.delay_ms(50);
    }
}
