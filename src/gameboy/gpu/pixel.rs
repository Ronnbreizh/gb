#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Pixel {
    Black = 0,
    DarkGrey = 1,
    LightGrey = 2,
    White = 3,
}

impl From<u8> for Pixel {
    fn from(value: u8) -> Pixel {
        match value {
            0 => Pixel::Black,
            1 => Pixel::DarkGrey,
            2 => Pixel::LightGrey,
            3 => Pixel::White,
            _ => panic!("Impossible pixel value"),
        }
    }
}

impl Pixel {
    /// Helper function to convert word + index into a Pixel
    pub fn from_bytes(low_value: u8, high_value: u8, index: usize) -> Pixel {
        let low_value_bit = (low_value << index) >> 7;
        let high_value_bit = (high_value << index) >> 7;
        let computed_value = (high_value_bit << 1) + low_value_bit;
        Pixel::from(computed_value)
    }

    pub fn to_rgb(self) -> [u8; 3] {
        // TODO
        match self {
            Pixel::Black => [0x00, 0x20, 0x00],
            Pixel::DarkGrey => [0x20, 0x40, 0x20],
            Pixel::LightGrey => [0x40, 0x80, 0x40],
            Pixel::White => [0xee, 0xff, 0xee],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black() {
        let pixel = Pixel::from_bytes(0x3C, 0x7E, 7);
        assert_eq!(pixel, Pixel::Black);
    }

    #[test]
    fn test_white() {
        let pixel = Pixel::from_bytes(0x3C, 0x7E, 5);
        assert_eq!(pixel, Pixel::White);
    }

    #[test]
    fn test_light_grey() {
        let pixel = Pixel::from_bytes(0x3C, 0x7E, 6);
        assert_eq!(pixel, Pixel::LightGrey);
    }
}
