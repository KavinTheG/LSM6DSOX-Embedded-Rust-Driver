#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Register {
    // Gyro Configuration Register
    CTRL2_G = 0x11,

    // Angular rate pitch axis (X) output
    OUTX_L_G = 0x22,
    OUTX_H_G = 0x23,

    // Angular rate roll axis (Y) output
    OUTY_L_G = 0x24,
    OUTY_H_G = 0x25,

    // Angular rate yaw axis (Z) output
    OUTZ_L_G = 0x26,
    OUTZ_H_G = 0x27,
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}