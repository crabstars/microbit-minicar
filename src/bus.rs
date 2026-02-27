use microbit::hal::twim::Twim;

pub const I2C_ADDR: u8 = 0x30;

pub type CarI2c = Twim<microbit::pac::TWIM0>;

pub fn write_reg(i2c: &mut CarI2c, reg: u8, value: u8) {
    let _ = i2c.write(I2C_ADDR, &[reg, value]);
}
