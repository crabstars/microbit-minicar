#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    board::Board,
    hal::{Timer, twim, twim::Twim},
};
use panic_halt as _;

const I2C_ADDR: u8 = 0x30;

fn write_reg(i2c: &mut Twim<microbit::pac::TWIM0>, reg: u8, value: u8) {
    let _ = i2c.write(I2C_ADDR, &[reg, value]);
}

fn motor_stop(i2c: &mut Twim<microbit::pac::TWIM0>) {
    // MiniCar extension mapping:
    // M1: reg 0x01 / 0x02, M2: reg 0x04 / 0x03
    write_reg(i2c, 0x01, 0);
    write_reg(i2c, 0x02, 0);
    write_reg(i2c, 0x03, 0);
    write_reg(i2c, 0x04, 0);
}

fn motor_forward(i2c: &mut Twim<microbit::pac::TWIM0>, speed: u8) {
    // Same behavior as MiniCar TS:
    // motor(M1, Forward, s) => reg 0x02=s, reg 0x01=0
    // motor(M2, Forward, s) => reg 0x03=s, reg 0x04=0
    write_reg(i2c, 0x01, 0);
    write_reg(i2c, 0x02, speed);
    write_reg(i2c, 0x04, 0);
    write_reg(i2c, 0x03, speed);
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // MakeCode uses the external micro:bit I2C bus (P19/P20).
    let mut i2c = Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        twim::Frequency::K100,
    );

    // Safety: always stop first (prevents stale motor state).
    motor_stop(&mut i2c);
    timer.delay_ms(100_u32);

    loop {
        motor_forward(&mut i2c, 255);
        timer.delay_ms(5000_u32);

        motor_stop(&mut i2c);
        timer.delay_ms(2000_u32);
    }
}
