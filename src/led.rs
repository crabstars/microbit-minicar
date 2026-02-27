use crate::bus::{CarI2c, write_reg};

#[derive(PartialEq, Clone, Copy)]
pub enum LedColor {
    Red = 1,
    Green = 2,
    Blue = 3,
    Cyan = 4,
    Purple = 5,
    White = 6,
    Yellow = 7,
    Black = 8,
}

#[derive(Clone, Copy)]
pub enum LedRgb {
    Led1 = 1,
    Led2 = 2,
}

#[repr(u8)]
enum RightLed {
    Red = 0x08,
    Green = 0x07,
    Blue = 0x06,
}

#[repr(u8)]
enum LeftLed {
    Red = 0x09,
    Green = 0x0A,
    Blue = 0x05,
}

fn color_to_pwm(color: LedColor) -> (u8, u8, u8) {
    match color {
        LedColor::Red => (0, 255, 255),
        LedColor::Green => (255, 0, 255),
        LedColor::Blue => (255, 255, 0),
        LedColor::Cyan => (255, 0, 0),
        LedColor::Purple => (0, 255, 0),
        LedColor::White => (0, 0, 0),
        LedColor::Yellow => (0, 0, 255),
        LedColor::Black => (255, 255, 255),
    }
}

pub fn set_color(i2c: &mut CarI2c, led: LedRgb, color: LedColor) {
    set_rgb(i2c, led, color_to_pwm(color));
}

pub fn set_rgb(i2c: &mut CarI2c, led: LedRgb, rgb: (u8, u8, u8)) {
    let (r, g, b) = rgb;

    match led {
        LedRgb::Led1 => {
            write_reg(i2c, RightLed::Red as u8, r);
            write_reg(i2c, RightLed::Green as u8, g);
            write_reg(i2c, RightLed::Blue as u8, b);
        }
        LedRgb::Led2 => {
            write_reg(i2c, LeftLed::Red as u8, r);
            write_reg(i2c, LeftLed::Green as u8, g);
            write_reg(i2c, LeftLed::Blue as u8, b);
        }
    }
}

pub fn disable(i2c: &mut CarI2c) {
    write_reg(i2c, RightLed::Red as u8, 255);
    write_reg(i2c, RightLed::Blue as u8, 255);
    write_reg(i2c, RightLed::Green as u8, 255);
    write_reg(i2c, LeftLed::Red as u8, 255);
    write_reg(i2c, LeftLed::Blue as u8, 255);
    write_reg(i2c, LeftLed::Green as u8, 255);
}
