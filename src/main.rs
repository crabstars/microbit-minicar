#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::twim::{self, Twim},
};
use microbit_minicar::{led, line_tracking, motor};
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

    let _ = motor::stop(&mut i2c);
    let _ = led::disable(&mut i2c);

    let mut left_sensor = board.edge.e12.into_pullup_input();
    let mut right_sensor = board.pins.p0_17.into_pullup_input();

    loop {
        match line_tracking::read(&mut left_sensor, &mut right_sensor) {
            Ok(state) => rprintln!("line sensor: {:?}", state),
            Err(err) => rprintln!("line sensor read failed: {:?}", err),
        }
    }
}
