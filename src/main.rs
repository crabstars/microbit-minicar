#![no_std]
#![no_main]

use core::hint::spin_loop;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;
use embedded_hal::digital::OutputPin;
use microbit::{
    board::Board,
    hal::{
        Timer,
        twim::{self, Twim},
    },
};
use nrf52833_hal::gpio::{
    Floating, Input, Output, PushPull,
    p0::{P0_01, P0_13},
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const I2C_ADDR: u8 = 0x30;

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

fn ultra(
    trigger: &mut P0_01<Output<PushPull>>,
    echo: &mut P0_13<Input<Floating>>,
    timer: &mut Timer<microbit::pac::TIMER0>,
    clock: &mut Timer<microbit::pac::TIMER1>,
) -> u32 {
    trigger.set_low();
    timer.delay_us(2);
    trigger.set_high();
    timer.delay_us(10);
    trigger.set_low();
    // spin_loop();
    clock.start(u32::MAX); // because we dont have a event we want to trigger when the
    // timer is reached, so we just set it to max value
    let start = clock.read();
    let mut now = clock.read();
    let mut elapsed = now.wrapping_sub(start);

    // echo is high true until the wave comes back
    while echo.is_high().ok().unwrap_or(true) && elapsed < 100_000 {
        // cancel after 100 ms
        now = clock.read();
        elapsed = now.wrapping_sub(start);
    }
    if elapsed > 100_000 {
        rprintln!("ERROR");
        return u32::MAX;
    }
    rprintln!("{:?}", elapsed);

    return elapsed / 58;
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // The TWIM instances share the same address space with instances of SPIM, SPIS, SPI, TWIS, and TWI. For example, TWIM0 conflicts with SPIM0, SPIS0, etc.; TWIM1 conflicts with SPIM1, SPIS1, etc. You need to make sure that conflicting instances are disabled before using Twim. Please refer to the product specification for more information (section 15.2 for nRF52832, section 6.1.2 for nRF52840).
    // TWI (“Two-Wire Interface”), TWIM (“Two-Wire Interface Master”) and TWIS (“Two-Wire Interface Slave”).
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut timer2 = Timer::new(board.TIMER1);

    // MakeCode uses the external micro:bit I2C bus (P19/P20).
    let mut i2c = Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        twim::Frequency::K100,
    );

    // Safety: always stop first (prevents stale motor state).
    motor_stop(&mut i2c);
    led_disable(&mut i2c);

    let mut trigger_pin = board
        .pins
        .p0_01
        .into_push_pull_output(nrf52833_hal::gpio::Level::Low);
    let mut echo_pin = board.pins.p0_13.into_floating_input();

    loop {
        let distance = ultra(&mut trigger_pin, &mut echo_pin, &mut timer, &mut timer2);
        if distance > 10 && distance < 100 {
            rprintln!("distance: {:?}", distance);
            led_set_color(&mut i2c, LedRgb::Led1, LedColor::Green);
        } else {
            led_set_color(&mut i2c, LedRgb::Led1, LedColor::Red);
        }
    }
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

    // loop {}
}
