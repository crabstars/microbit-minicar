use crate::bus::{CarI2c, write_reg};

#[derive(Clone, Copy)]
pub enum Direction {
    Forward = 1,
    Backward = 2,
}

#[derive(Clone, Copy)]
pub enum Motor {
    A = 1,
    B = 2,
}

pub fn stop(i2c: &mut CarI2c) {
    write_reg(i2c, 0x01, 0);
    write_reg(i2c, 0x02, 0);
    write_reg(i2c, 0x03, 0);
    write_reg(i2c, 0x04, 0);
}

pub fn set(i2c: &mut CarI2c, speed: u8, motor: Motor, direction: Direction) {
    match direction {
        Direction::Forward => match motor {
            Motor::A => {
                write_reg(i2c, 0x01, 0);
                write_reg(i2c, 0x02, speed);
            }
            Motor::B => {
                write_reg(i2c, 0x03, speed);
                write_reg(i2c, 0x04, 0);
            }
        },
        Direction::Backward => match motor {
            Motor::A => {
                write_reg(i2c, 0x01, speed);
                write_reg(i2c, 0x02, 0);
            }
            Motor::B => {
                write_reg(i2c, 0x03, 0);
                write_reg(i2c, 0x04, speed);
            }
        },
    }
}
