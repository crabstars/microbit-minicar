#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    board::Board,
    hal::{
        twim::{self, Twim},
        Timer,
    },
};
use microbit_minicar::led::{self, LedColor, LedRgb};
use microbit_minicar::motor;
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

    let _ = motor::stop(&mut i2c);
    let _ = led::disable(&mut i2c);

    let colors = [
        LedColor::Red,
        LedColor::Green,
        LedColor::Blue,
        LedColor::Yellow,
        LedColor::Cyan,
        LedColor::Purple,
        LedColor::White,
        LedColor::Black,
    ];

    loop {
        for color in colors {
            let _ = led::set_color(&mut i2c, LedRgb::Led1, color);
            let _ = led::set_color(&mut i2c, LedRgb::Led2, color);
            timer.delay_ms(500);
        }
    }
}
