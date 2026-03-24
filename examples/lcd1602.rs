#![no_std]
#![no_main]

use car_core::{
    lcd1602::{DEFAULT_ADDR, Lcd1602},
    led, motor,
};
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
use rtt_target::{rprintln, rtt_init_print};

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

    motor::stop(&mut i2c);
    led::disable(&mut i2c);

    let mut lcd = Lcd1602::new(DEFAULT_ADDR);

    if lcd.init(&mut i2c, &mut timer).is_ok() {
        let _ = lcd.write_line(&mut i2c, &mut timer, 0, "MiniCar LCD");
        let _ = lcd.write_line(&mut i2c, &mut timer, 1, "Hello from Rust");
        rprintln!("LCD initialized at 0x27");
    } else {
        rprintln!("LCD init failed, check wiring/address");
    }

    loop {
        timer.delay_ms(1_000);
    }
}
