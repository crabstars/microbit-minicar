use embedded_hal::{delay::DelayNs, i2c::I2c};

pub const DEFAULT_ADDR: u8 = 0x27;

const BACKLIGHT: u8 = 0x08;
const ENABLE: u8 = 0x04;
const RS: u8 = 0x01;

pub struct Lcd1602 {
    address: u8,
    backlight: u8,
}

impl Lcd1602 {
    pub const fn new(address: u8) -> Self {
        Self {
            address,
            backlight: BACKLIGHT,
        }
    }

    pub fn init<I2C, D>(&mut self, i2c: &mut I2C, delay: &mut D) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        delay.delay_ms(50);
        self.write_4bits(i2c, 0x30, false, delay)?;
        delay.delay_ms(5);
        self.write_4bits(i2c, 0x30, false, delay)?;
        delay.delay_ms(5);
        self.write_4bits(i2c, 0x30, false, delay)?;
        delay.delay_ms(1);
        self.write_4bits(i2c, 0x20, false, delay)?;

        self.command(i2c, 0x28, delay)?;
        self.command(i2c, 0x08, delay)?;
        self.clear(i2c, delay)?;
        self.command(i2c, 0x06, delay)?;
        self.command(i2c, 0x0c, delay)?;

        Ok(())
    }

    pub fn clear<I2C, D>(&self, i2c: &mut I2C, delay: &mut D) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        self.command(i2c, 0x01, delay)
    }

    pub fn set_backlight<I2C>(&mut self, i2c: &mut I2C, enabled: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.backlight = if enabled { BACKLIGHT } else { 0 };
        self.write_expander(i2c, self.backlight)
    }

    pub fn set_cursor<I2C, D>(
        &self,
        i2c: &mut I2C,
        delay: &mut D,
        col: u8,
        row: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        let row_offsets = [0x00, 0x40];
        let row = usize::from(row.min(1));
        let col = col.min(15);
        self.command(i2c, 0x80 | (col + row_offsets[row]), delay)
    }

    pub fn write_str<I2C, D>(
        &self,
        i2c: &mut I2C,
        delay: &mut D,
        text: &str,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        for byte in text.bytes() {
            self.data(i2c, byte, delay)?;
        }

        Ok(())
    }

    pub fn write_line<I2C, D>(
        &self,
        i2c: &mut I2C,
        delay: &mut D,
        row: u8,
        text: &str,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        self.set_cursor(i2c, delay, 0, row)?;

        let mut written = 0;
        for byte in text.bytes().take(16) {
            self.data(i2c, byte, delay)?;
            written += 1;
        }

        while written < 16 {
            self.data(i2c, b' ', delay)?;
            written += 1;
        }

        Ok(())
    }

    fn command<I2C, D>(&self, i2c: &mut I2C, value: u8, delay: &mut D) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        self.send(i2c, value, false, delay)?;

        if matches!(value, 0x01 | 0x02) {
            delay.delay_ms(2);
        }

        Ok(())
    }

    fn data<I2C, D>(&self, i2c: &mut I2C, value: u8, delay: &mut D) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        self.send(i2c, value, true, delay)
    }

    fn send<I2C, D>(
        &self,
        i2c: &mut I2C,
        value: u8,
        is_data: bool,
        delay: &mut D,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        self.write_4bits(i2c, value & 0xf0, is_data, delay)?;
        self.write_4bits(i2c, (value << 4) & 0xf0, is_data, delay)
    }

    fn write_4bits<I2C, D>(
        &self,
        i2c: &mut I2C,
        value: u8,
        is_data: bool,
        delay: &mut D,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        let state = value | self.backlight | if is_data { RS } else { 0 };
        self.write_expander(i2c, state)?;
        self.pulse_enable(i2c, state, delay)
    }

    fn pulse_enable<I2C, D>(
        &self,
        i2c: &mut I2C,
        value: u8,
        delay: &mut D,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        D: DelayNs,
    {
        self.write_expander(i2c, value | ENABLE)?;
        delay.delay_us(1);
        self.write_expander(i2c, value & !ENABLE)?;
        delay.delay_us(50);
        Ok(())
    }

    fn write_expander<I2C>(&self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        i2c.write(self.address, &[value])
    }
}

impl Default for Lcd1602 {
    fn default() -> Self {
        Self::new(DEFAULT_ADDR)
    }
}
