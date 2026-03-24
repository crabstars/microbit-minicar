#![no_std]
#![no_main]

use car_core::{led, motor};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::hal::Timer;
use microbit::{
    board::Board,
    hal::twim::{self, Twim},
};
use nrf52833_hal::{
    self,
    pwm::{Channel, Pwm},
    time::Hertz,
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

    let mut timer = Timer::new(board.TIMER0);

    let servo_pin = board
        .edge
        .e02
        .into_push_pull_output(nrf52833_hal::gpio::Level::Low)
        .degrade();
    let pwm = Pwm::new(board.PWM0);
    pwm.set_output_pin(Channel::C0, servo_pin);
    pwm.set_period(Hertz(50));

    let max = pwm.max_duty();
    const SWEEP_STEPS: u16 = 100;
    const STEP_DELAY_MS: u32 = 10;

    pwm.enable();
    pwm.set_duty_on(Channel::C0, 0);

    // TODO: is not doing a 180 degree
    loop {
        for step in 1..=SWEEP_STEPS {
            let duty = (u32::from(max) * u32::from(step) / u32::from(SWEEP_STEPS)) as u16;
            rprintln!("up {:?}", duty);
            pwm.set_duty_on(Channel::C0, duty);
            timer.delay_ms(STEP_DELAY_MS);
        }

        for step in (0..SWEEP_STEPS).rev() {
            let duty = (u32::from(max) * u32::from(step) / u32::from(SWEEP_STEPS)) as u16;
            rprintln!("down {:?}", duty);
            pwm.set_duty_on(Channel::C0, duty);
            timer.delay_ms(STEP_DELAY_MS);
        }
        break;
    }
    loop {}
}
