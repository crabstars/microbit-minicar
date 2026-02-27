use embedded_hal::digital::InputPin;
use nrf52833_hal::gpio::p0::{P0_12, P0_17};
use nrf52833_hal::gpio::{Input, PullUp};

#[derive(Debug, Clone, Copy)]
pub enum LineTrackingSensor {
    Both = 0,
    Left = 1,
    Right = 2,
    None = 3,
    Unknown = 4,
}

pub fn read(p12: &mut P0_12<Input<PullUp>>, p17: &mut P0_17<Input<PullUp>>) -> LineTrackingSensor {
    let val = (p12.is_high().unwrap() as u8) | (p17.is_high().unwrap() as u8) << 1;
    match val {
        0 => LineTrackingSensor::Both,
        1 => LineTrackingSensor::Left,
        2 => LineTrackingSensor::Right,
        3 => LineTrackingSensor::None,
        _ => LineTrackingSensor::Unknown,
    }
}
