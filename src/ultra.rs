use core::hint::spin_loop;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::Timer;

const TIMEOUT_US: u32 = 30_000;
const MIN_VALID_PULSE_US: u32 = 120;

pub fn measure_cm<Trig, Echo>(
    trigger: &mut Trig,
    echo: &mut Echo,
    pulse_timer: &mut Timer<microbit::pac::TIMER0>,
    clock: &mut Timer<microbit::pac::TIMER1>,
) -> u32
where
    Trig: OutputPin,
    Echo: InputPin,
{
    trigger.set_low().ok();
    pulse_timer.delay_us(2);
    trigger.set_high().ok();
    pulse_timer.delay_us(10);
    trigger.set_low().ok();

    clock.start(u32::MAX);
    let t_idle = clock.read();

    while echo.is_high().unwrap() {
        if clock.read().wrapping_sub(t_idle) > TIMEOUT_US {
            return u32::MAX;
        }
        spin_loop();
    }

    let t_wait_rise = clock.read();
    while echo.is_low().unwrap() {
        if clock.read().wrapping_sub(t_wait_rise) > TIMEOUT_US {
            return u32::MAX;
        }
        spin_loop();
    }
    let start = clock.read();

    while echo.is_high().unwrap() {
        if clock.read().wrapping_sub(start) > TIMEOUT_US {
            return u32::MAX;
        }
        spin_loop();
    }
    let end = clock.read();

    let pulse_us = end.wrapping_sub(start);
    if pulse_us < MIN_VALID_PULSE_US {
        return u32::MAX;
    }

    pulse_us / 58
}
