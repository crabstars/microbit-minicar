#![no_std]
#![no_main]

use car_core::{led, line_tracking, motor};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::twim::{self, Twim},
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut i2c = Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        twim::Frequency::K100,
    );

    motor::stop(&mut i2c);
    led::disable(&mut i2c);

    let mut left_sensor = board.edge.e12.into_pullup_input();
    let mut right_sensor = board.pins.p0_17.into_pullup_input();

    loop {
        let state = line_tracking::read(&mut left_sensor, &mut right_sensor);
        rprintln!("line sensor: {:?}", state);
    }
}
