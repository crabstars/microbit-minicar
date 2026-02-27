#![no_std]
#![no_main]

use car_core::led::{self, LedColor, LedRgb};
use car_core::motor;
use car_core::ultra;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        twim::{self, Twim},
        Timer,
    },
};
use nrf52833_hal::gpio::Level;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut pulse_timer = Timer::new(board.TIMER0);
    let mut clock = Timer::new(board.TIMER1);

    let mut i2c = Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        twim::Frequency::K100,
    );

    motor::stop(&mut i2c);
    led::disable(&mut i2c);

    let mut trigger_pin = board.pins.p0_01.into_push_pull_output(Level::Low);
    let mut echo_pin = board.pins.p0_13.into_floating_input();
    let mut current = LedColor::Red;
    led::set_color(&mut i2c, LedRgb::Led1, current);

    loop {
        let distance = ultra::measure_cm(
            &mut trigger_pin,
            &mut echo_pin,
            &mut pulse_timer,
            &mut clock,
        );
        rprintln!("distance: {}cm", distance);

        let next = if distance != u32::MAX && distance < 15 {
            LedColor::Green
        } else {
            LedColor::Red
        };

        if next != current {
            current = next;
            led::set_color(&mut i2c, LedRgb::Led1, current);
        }
    }
}
