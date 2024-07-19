pub fn get_low_12_bits(value: u16) -> u16 {
    value & 0xFFF
}

pub fn get_low_4_bits_of_high_byte(value: u16) -> u8 {
    ((value & 0x0F00) >> 8) as u8
}

pub fn get_high_4_bits_of_low_byte(value: u16) -> u8 {
    ((value & 0x00F0) >> 4) as u8 
}

pub fn get_lowest_4_bits(value: u16) -> u8 {
    (value & 0x000F) as u8
}
pub fn get_low_byte(value: u16) -> u8 {
    (value & 0x00FF) as u8
}

