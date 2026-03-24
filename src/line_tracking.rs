use embedded_hal::digital::InputPin;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineTrackingSensor {
    Both = 0,
    Left = 1,
    Right = 2,
    None = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineTrackingError<LeftError, RightError> {
    Left(LeftError),
    Right(RightError),
}

pub fn read<LeftPin, RightPin>(
    left: &mut LeftPin,
    right: &mut RightPin,
) -> Result<LineTrackingSensor, LineTrackingError<LeftPin::Error, RightPin::Error>>
where
    LeftPin: InputPin,
    RightPin: InputPin,
{
    let left_high = left.is_high().map_err(LineTrackingError::Left)? as u8;
    let right_high = right.is_high().map_err(LineTrackingError::Right)? as u8;

    Ok(match left_high | (right_high << 1) {
        0 => LineTrackingSensor::Both,
        1 => LineTrackingSensor::Left,
        2 => LineTrackingSensor::Right,
        _ => LineTrackingSensor::None,
    })
}
