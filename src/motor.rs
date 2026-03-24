use embedded_hal::i2c::I2c;

use crate::bus::write_reg;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Forward = 1,
    Backward = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Motor {
    A = 1,
    B = 2,
}

pub fn stop<I2C>(i2c: &mut I2C) -> Result<(), I2C::Error>
where
    I2C: I2c,
{
    write_reg(i2c, 0x01, 0)?;
    write_reg(i2c, 0x02, 0)?;
    write_reg(i2c, 0x03, 0)?;
    write_reg(i2c, 0x04, 0)?;
    Ok(())
}

pub fn set<I2C>(
    i2c: &mut I2C,
    speed: u8,
    motor: Motor,
    direction: Direction,
) -> Result<(), I2C::Error>
where
    I2C: I2c,
{
    match direction {
        Direction::Forward => match motor {
            Motor::A => {
                write_reg(i2c, 0x01, 0)?;
                write_reg(i2c, 0x02, speed)?;
            }
            Motor::B => {
                write_reg(i2c, 0x03, speed)?;
                write_reg(i2c, 0x04, 0)?;
            }
        },
        Direction::Backward => match motor {
            Motor::A => {
                write_reg(i2c, 0x01, speed)?;
                write_reg(i2c, 0x02, 0)?;
            }
            Motor::B => {
                write_reg(i2c, 0x03, 0)?;
                write_reg(i2c, 0x04, speed)?;
            }
        },
    }

    Ok(())
}
