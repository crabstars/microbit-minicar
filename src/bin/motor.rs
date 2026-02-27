#![no_std]
#![no_main]

use car_core::led;
use car_core::motor::{self, Direction, Motor};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    board::Board,
    hal::{
        Timer,
        twim::{self, Twim},
    },
};
use panic_halt as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut i2c = Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        twim::Frequency::K100,
    );

    led::disable(&mut i2c);
    motor::stop(&mut i2c);

    loop {
        motor::set(&mut i2c, 90, Motor::A, Direction::Forward);
        motor::set(&mut i2c, 90, Motor::B, Direction::Forward);
        timer.delay_ms(1_500);

        motor::set(&mut i2c, 90, Motor::A, Direction::Backward);
        motor::set(&mut i2c, 90, Motor::B, Direction::Backward);
        timer.delay_ms(1_500);

        motor::stop(&mut i2c);
        timer.delay_ms(1_000);
    }
}
