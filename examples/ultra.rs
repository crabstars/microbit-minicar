#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        twim::{self, Twim},
        Timer,
    },
};
use microbit_minicar::led::{self, LedColor, LedRgb};
use microbit_minicar::motor;
use microbit_minicar::ultra;
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

    let _ = motor::stop(&mut i2c);
    let _ = led::disable(&mut i2c);

    let mut trigger_pin = board.pins.p0_01.into_push_pull_output(Level::Low);
    let mut echo_pin = board.pins.p0_13.into_floating_input();
    let mut current = LedColor::Red;
    let _ = led::set_color(&mut i2c, LedRgb::Led1, current);
    clock.start(u32::MAX);

    loop {
        let distance = ultra::measure_cm(
            &mut trigger_pin,
            &mut echo_pin,
            &mut pulse_timer,
            &mut || clock.read(),
        );
        let next = match distance {
            Ok(Some(distance_cm)) => {
                rprintln!("distance: {}cm", distance_cm);
                if distance_cm < 15 {
                    LedColor::Green
                } else {
                    LedColor::Red
                }
            }
            Ok(None) => {
                rprintln!("distance: timeout");
                LedColor::Red
            }
            Err(err) => {
                rprintln!("distance read failed: {:?}", err);
                LedColor::Red
            }
        };

        if next != current {
            current = next;
            let _ = led::set_color(&mut i2c, LedRgb::Led1, current);
        }
    }
}
