use core::hint::spin_loop;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};

const TIMEOUT_US: u32 = 30_000;
const MIN_VALID_PULSE_US: u32 = 120;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UltrasonicError<TriggerError, EchoError> {
    Trigger(TriggerError),
    Echo(EchoError),
}

pub fn measure_cm<Trig, Echo, Delay, Clock>(
    trigger: &mut Trig,
    echo: &mut Echo,
    pulse_delay: &mut Delay,
    now_us: &mut Clock,
) -> Result<Option<u32>, UltrasonicError<Trig::Error, Echo::Error>>
where
    Trig: OutputPin,
    Echo: InputPin,
    Delay: DelayNs,
    Clock: FnMut() -> u32,
{
    trigger.set_low().map_err(UltrasonicError::Trigger)?;
    pulse_delay.delay_us(2);
    trigger.set_high().map_err(UltrasonicError::Trigger)?;
    pulse_delay.delay_us(10);
    trigger.set_low().map_err(UltrasonicError::Trigger)?;

    let t_idle = now_us();

    while echo.is_high().map_err(UltrasonicError::Echo)? {
        if now_us().wrapping_sub(t_idle) > TIMEOUT_US {
            return Ok(None);
        }
        spin_loop();
    }

    let t_wait_rise = now_us();
    while echo.is_low().map_err(UltrasonicError::Echo)? {
        if now_us().wrapping_sub(t_wait_rise) > TIMEOUT_US {
            return Ok(None);
        }
        spin_loop();
    }

    let start = now_us();
    while echo.is_high().map_err(UltrasonicError::Echo)? {
        if now_us().wrapping_sub(start) > TIMEOUT_US {
            return Ok(None);
        }
        spin_loop();
    }

    let pulse_us = now_us().wrapping_sub(start);
    if pulse_us < MIN_VALID_PULSE_US {
        return Ok(None);
    }

    Ok(Some(pulse_us / 58))
}
